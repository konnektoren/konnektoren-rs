use konnektoren_rs::core::challenges::error::ChallengeError;
use konnektoren_rs::core::challenges::task_pattern::TaskPattern;
use konnektoren_rs::core::challenges::{
    Attempt, ChallengeConfig, ChallengeDefinition, ChallengeFactory, Performance, Playable,
};
use konnektoren_rs::core::commands::{ChallengeCommand, Command, CommandError, GameCommand};
use konnektoren_rs::core::controller::{
    ChallengeFinishPlugin, GameController, GameControllerTrait, GameXpPlugin,
};
use konnektoren_rs::core::game::{Game, GamePath, GameState};
use konnektoren_rs::core::persistence::MemoryPersistence;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// An application-owned challenge type that core knows nothing about.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum AppChallenge {
    Melody { id: String, notes: Vec<u8> },
}

/// How the app presents its melodies; chosen per game-path entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum AppVariant {
    Staff,
    Keyboard,
}

type Notes = Vec<u8>;

impl ChallengeDefinition for AppChallenge {
    type Result = Notes;

    fn id(&self) -> &str {
        match self {
            Self::Melody { id, .. } => id,
        }
    }

    fn of_tasks(&self, tasks: &TaskPattern) -> Self {
        match self {
            Self::Melody { id, notes } => Self::Melody {
                id: id.clone(),
                notes: tasks.select_items(notes),
            },
        }
    }
}

impl Performance<Notes> for AppChallenge {
    fn performance(&self, result: &Notes) -> u32 {
        let Self::Melody { notes, .. } = self;
        let correct = notes.iter().zip(result).filter(|(a, b)| a == b).count();
        (100 * correct / notes.len().max(1)) as u32
    }
}

impl Playable for AppChallenge {
    type Input = u8;
    type Variant = AppVariant;

    fn result_len(result: &Notes) -> usize {
        result.len()
    }

    fn default_input(&self, _task_index: usize) -> Result<Option<u8>, ChallengeError> {
        Ok(Some(0))
    }

    fn record_input(&self, result: &mut Option<Notes>, input: u8) -> Result<(), ChallengeError> {
        result.get_or_insert_with(Vec::new).push(input);
        Ok(())
    }

    fn try_performance(&self, result: &Notes) -> Result<u32, ChallengeError> {
        Ok(self.performance(result))
    }

    fn solve_task(
        challenge: &mut Attempt<Self>,
        input: u8,
        task_index: usize,
    ) -> Result<bool, ChallengeError> {
        let Self::Melody { notes, .. } = &challenge.challenge_type;
        let expected = *notes
            .get(task_index)
            .ok_or_else(|| ChallengeError::InvalidInput(format!("no task {task_index}")))?;
        let result = challenge.challenge_result.get_or_insert_with(Vec::new);
        if task_index < result.len() {
            result[task_index] = input;
        } else {
            result.push(input);
        }
        Ok(input == expected)
    }
}

fn melody(id: &str, notes: &[u8]) -> AppChallenge {
    AppChallenge::Melody {
        id: id.into(),
        notes: notes.to_vec(),
    }
}

fn config<V>(id: &str, challenge: &str, tasks: usize) -> ChallengeConfig<V> {
    ChallengeConfig {
        id: id.into(),
        name: id.into(),
        challenge: challenge.into(),
        tasks: TaskPattern::Exact(tasks),
        ..ChallengeConfig::default()
    }
    .with_variant(None)
}

fn game() -> Game<AppChallenge> {
    Game {
        game_paths: vec![GamePath {
            id: "music".into(),
            name: "Music".into(),
            challenges: vec![
                config("scale-run", "scale", 3),
                config("arpeggio-run", "arpeggio", 2),
            ],
            map: None,
        }],
        challenge_factory: ChallengeFactory {
            challenge_types: vec![
                melody("scale", &[60, 62, 64]),
                melody("arpeggio", &[60, 64]),
            ],
        },
        challenge_history: Default::default(),
        xp: 0,
    }
}

#[test]
fn external_game_loads_plays_saves_and_restores() -> Result<(), Box<dyn std::error::Error>> {
    let persistence = Arc::new(MemoryPersistence::<AppChallenge>::new(GameState::new(
        game(),
    )));
    let mut controller = GameController::new(game(), persistence.clone());
    controller.register_plugin(Arc::new(ChallengeFinishPlugin));
    controller.register_plugin(Arc::new(GameXpPlugin));
    let controller = controller.init();

    // Load: the state starts at the first external challenge with only its selected tasks.
    {
        let state = controller.game_state().lock().unwrap();
        assert_eq!(
            state.challenge.challenge_type,
            melody("scale", &[60, 62, 64])
        );
        assert!(state.challenge.challenge_result.is_none());
    }

    // Play: two right answers, then the last task is skipped and padded with the default.
    controller.publish_command(Command::Challenge(ChallengeCommand::Solve(60)));
    controller.publish_command(Command::Challenge(ChallengeCommand::Solve(62)));
    controller.publish_command(Command::Challenge(ChallengeCommand::NextTask));
    {
        let state = controller.game_state().lock().unwrap();
        assert_eq!(state.current_task_index, 2);
        assert_eq!(state.challenge.challenge_result, Some(vec![60, 62]));
    }
    controller.publish_command(Command::Challenge(ChallengeCommand::Solve(64)));

    // Finish: history and XP are updated by the generic plugins, then the state is saved.
    let result = controller
        .game_state()
        .lock()
        .unwrap()
        .challenge
        .challenge_result
        .clone();
    assert_eq!(result, Some(vec![60, 62, 64]));
    controller.publish_command(Command::Challenge(ChallengeCommand::Finish(result)));
    {
        let state = controller.game_state().lock().unwrap();
        assert_eq!(state.game.challenge_history.len(), 1);
        assert_eq!(state.game.xp, 10);
    }

    // Move on, then restore the saved state from persistence.
    controller.publish_command(Command::Game(GameCommand::NextChallenge));
    assert_eq!(
        controller
            .game_state()
            .lock()
            .unwrap()
            .challenge
            .challenge_type,
        melody("arpeggio", &[60, 64])
    );
    controller.load_game_state()?;
    let state = controller.game_state().lock().unwrap();
    assert_eq!(state.current_challenge_index, 0);
    assert_eq!(state.game.challenge_history.len(), 1);
    assert_eq!(state.game.xp, 10);

    // The saved state survives serialization with the external types intact.
    let json = serde_json::to_string(&*state)?;
    let restored: GameState<AppChallenge> = serde_json::from_str(&json)?;
    assert_eq!(restored, *state);
    Ok(())
}

#[test]
fn external_game_reports_unsupported_and_invalid_input() {
    let mut state = GameState::new(game());

    // This type does not present numbered options.
    let option = ChallengeCommand::<AppChallenge>::SolveOption(0);
    assert!(matches!(
        konnektoren_rs::core::commands::CommandTrait::execute(&option, &mut state),
        Err(
            konnektoren_rs::core::commands::CommandError::ChallengeError(
                ChallengeError::InvalidChallengeType
            )
        )
    ));

    // Solving past the selected tasks reports the error instead of scoring zero.
    state.current_task_index = 9;
    let solve = ChallengeCommand::<AppChallenge>::Solve(60);
    assert!(konnektoren_rs::core::commands::CommandTrait::execute(&solve, &mut state).is_err());
}

/// A second app-owned type, to combine with the first in a family.
#[derive(Clone, Debug, PartialEq)]
struct Quiz {
    answer: bool,
}

impl ChallengeDefinition for Quiz {
    type Result = Vec<bool>;

    fn id(&self) -> &str {
        "quiz"
    }

    fn of_tasks(&self, _tasks: &TaskPattern) -> Self {
        self.clone()
    }
}

impl Playable for Quiz {
    type Input = bool;
    type Variant = ();

    fn result_len(result: &Vec<bool>) -> usize {
        result.len()
    }

    fn default_input(&self, _task_index: usize) -> Result<Option<bool>, ChallengeError> {
        Ok(None)
    }

    fn record_input(
        &self,
        result: &mut Option<Vec<bool>>,
        input: bool,
    ) -> Result<(), ChallengeError> {
        result.get_or_insert_with(Vec::new).push(input);
        Ok(())
    }

    fn try_performance(&self, result: &Vec<bool>) -> Result<u32, ChallengeError> {
        Ok(if result.first() == Some(&self.answer) {
            100
        } else {
            0
        })
    }

    fn solve_task(
        challenge: &mut Attempt<Self>,
        input: bool,
        _task_index: usize,
    ) -> Result<bool, ChallengeError> {
        challenge
            .challenge_result
            .get_or_insert_with(Vec::new)
            .push(input);
        Ok(input == challenge.challenge_type.answer)
    }
}

#[test]
fn family_dispatches_input_and_rejects_mismatched_branches() {
    use konnektoren_rs::core::challenges::{
        ChallengeFamily, ChallengeFamilyInput, ChallengeFamilyResult,
    };
    use konnektoren_rs::core::commands::CommandTrait;
    type Family = ChallengeFamily<AppChallenge, Quiz>;

    let mut state = GameState::new(Game {
        game_paths: vec![GamePath {
            id: "mixed".into(),
            name: "Mixed".into(),
            challenges: vec![config("scale-run", "scale", 3)],
            map: None,
        }],
        challenge_factory: ChallengeFactory {
            challenge_types: vec![ChallengeFamily::First(melody("scale", &[60, 62, 64]))],
        },
        challenge_history: Default::default(),
        xp: 0,
    });

    // Matching input is solved by the right branch and stored under that branch.
    let solve = ChallengeCommand::<Family>::Solve(ChallengeFamilyInput::First(60));
    solve.execute(&mut state).unwrap();
    assert_eq!(
        state.challenge.challenge_result,
        Some(ChallengeFamilyResult::First(vec![60]))
    );

    // Input for the other branch is an explicit error and changes nothing.
    let wrong = ChallengeCommand::<Family>::Solve(ChallengeFamilyInput::Second(true));
    assert!(matches!(
        wrong.execute(&mut state),
        Err(CommandError::ChallengeError(ChallengeError::FamilyMismatch))
    ));
    assert_eq!(state.current_task_index, 1);
    assert_eq!(
        state.challenge.challenge_result,
        Some(ChallengeFamilyResult::First(vec![60]))
    );

    // A finish result from the other branch is rejected instead of scored as zero.
    let finish =
        ChallengeCommand::<Family>::Finish(Some(ChallengeFamilyResult::Second(vec![true])));
    assert!(matches!(
        finish.execute(&mut state),
        Err(CommandError::ChallengeError(ChallengeError::FamilyMismatch))
    ));
    let finish = ChallengeCommand::<Family>::Finish(Some(ChallengeFamilyResult::First(vec![60])));
    finish.execute(&mut state).unwrap();
}

#[test]
fn commands_for_included_types_drive_the_larger_game() {
    use konnektoren_rs::core::challenges::{
        ChallengeFamily, ChallengeFamilyInput, ChallengeFamilyResult, ChallengeResult,
        ChallengeType,
    };
    use konnektoren_rs::core::commands::CommandTrait;
    type Inner = ChallengeFamily<AppChallenge, Quiz>;
    type Outer = ChallengeFamily<ChallengeType, Inner>;

    let builtin = ChallengeType::default();
    let mut state = GameState::new(Game {
        game_paths: vec![GamePath {
            id: "mixed".into(),
            name: "Mixed".into(),
            challenges: vec![config("konnektoren-run", builtin.id(), 2)],
            map: None,
        }],
        challenge_factory: ChallengeFactory {
            challenge_types: vec![ChallengeFamily::First(builtin)],
        },
        challenge_history: Default::default(),
        xp: 0,
    });

    // A command made by core's default UI for a built-in challenge, embedded as the
    // larger type's command.
    let builtin_branch = Outer::first_embedding();
    Command::<ChallengeType>::Challenge(ChallengeCommand::SolveOption(0))
        .embed(&builtin_branch)
        .execute(&mut state)
        .unwrap();
    assert!(matches!(
        state.challenge.challenge_result,
        Some(ChallengeFamilyResult::First(ChallengeResult::MultipleChoice(ref answers)))
            if answers.len() == 1
    ));
    let finish = Command::<ChallengeType>::Challenge(ChallengeCommand::Finish(Some(
        ChallengeResult::MultipleChoice(vec![]),
    )))
    .embed(&builtin_branch);
    finish.execute(&mut state).unwrap();

    // Embedding composes through nested families.
    let app_branch = Inner::first_embedding().then(&Outer::second_embedding());
    let input = app_branch.input(60);
    assert_eq!(
        input,
        ChallengeFamilyInput::Second(ChallengeFamilyInput::First(60))
    );
    assert_eq!(
        app_branch.result(vec![60]),
        ChallengeFamilyResult::Second(ChallengeFamilyResult::First(vec![60]))
    );
    // App-branch input against the built-in challenge is an explicit mismatch.
    let wrong = ChallengeCommand::<Outer>::Solve(input);
    assert!(matches!(
        wrong.execute(&mut state),
        Err(CommandError::ChallengeError(ChallengeError::FamilyMismatch))
    ));
}

fn family_controller()
-> Arc<GameController<konnektoren_rs::core::challenges::ChallengeFamily<AppChallenge, Quiz>>> {
    use konnektoren_rs::core::challenges::ChallengeFamily;
    let game = Game {
        game_paths: vec![GamePath {
            id: "mixed".into(),
            name: "Mixed".into(),
            challenges: vec![config("scale-run", "scale", 3)],
            map: None,
        }],
        challenge_factory: ChallengeFactory {
            challenge_types: vec![ChallengeFamily::First(melody("scale", &[60, 62, 64]))],
        },
        challenge_history: Default::default(),
        xp: 0,
    };
    let persistence = Arc::new(MemoryPersistence::new(GameState::new(game.clone())));
    let mut controller = GameController::new(game, persistence);
    controller.register_plugin(Arc::new(ChallengeFinishPlugin));
    controller.register_plugin(Arc::new(GameXpPlugin));
    controller.init()
}

#[test]
fn published_finish_with_a_wrong_family_branch_changes_nothing() {
    use konnektoren_rs::core::challenges::ChallengeFamilyResult;
    let controller = family_controller();
    let before = controller.game_state().lock().unwrap().clone();

    controller.publish_command(Command::Challenge(ChallengeCommand::Finish(Some(
        ChallengeFamilyResult::Second(vec![true]),
    ))));

    let state = controller.game_state().lock().unwrap();
    assert_eq!(*state, before);
    assert!(state.game.challenge_history.is_empty());
    assert_eq!(state.game.xp, 0);
}

#[test]
fn published_finish_with_a_mismatched_builtin_result_changes_nothing() {
    use konnektoren_rs::core::challenges::{ChallengeResult, ChallengeType};
    use konnektoren_rs::core::commands::CommandTrait;

    let persistence = Arc::new(MemoryPersistence::default());
    let controller = GameController::new(Game::default(), persistence).init();
    let before = controller.game_state().lock().unwrap().clone();
    let wrong = Command::Challenge(ChallengeCommand::Finish(Some(ChallengeResult::Dialog(
        vec![],
    ))));

    // Directly executed, the command reports the mismatch.
    let mut copy = before.clone();
    assert!(matches!(
        CommandTrait::<ChallengeType>::execute(&wrong, &mut copy),
        Err(CommandError::ChallengeError(
            ChallengeError::InvalidChallengeType
        ))
    ));
    // Published, the plugins never see it.
    controller.publish_command(wrong);
    let state = controller.game_state().lock().unwrap();
    assert_eq!(*state, before);
    assert!(state.game.challenge_history.is_empty());
    assert_eq!(state.game.xp, 0);
}

#[test]
fn finishing_the_same_attempt_twice_awards_xp_and_history_once() {
    let controller = {
        let game = game();
        let persistence = Arc::new(MemoryPersistence::new(GameState::new(game.clone())));
        let mut controller = GameController::new(game, persistence);
        controller.register_plugin(Arc::new(ChallengeFinishPlugin));
        controller.register_plugin(Arc::new(GameXpPlugin));
        controller.init()
    };
    let finish = || Command::Challenge(ChallengeCommand::Finish(Some(vec![60, 62, 64])));

    controller.publish_command(finish());
    controller.publish_command(finish());
    {
        let state = controller.game_state().lock().unwrap();
        assert_eq!(state.game.xp, 10);
        assert_eq!(state.game.challenge_history.len(), 1);
    }

    // A fresh attempt of the same challenge is a new transition.
    controller.publish_command(Command::Game(GameCommand::NextChallenge));
    controller.publish_command(Command::Game(GameCommand::PreviousChallenge));
    controller.publish_command(finish());
    let state = controller.game_state().lock().unwrap();
    assert_eq!(state.game.xp, 20);
    assert_eq!(state.game.challenge_history.len(), 2);
}

#[test]
fn app_owned_variants_choose_presentation_and_survive_saving()
-> Result<(), Box<dyn std::error::Error>> {
    use konnektoren_rs::core::challenges::{ChallengeFamily, ChallengeFamilyVariant};

    // The app's own variant is chosen in the game path and reaches the active attempt.
    let mut game = game();
    game.game_paths[0].challenges[0].variant = Some(AppVariant::Keyboard);
    let state = GameState::new(game);
    assert_eq!(
        state.challenge.challenge_config.variant,
        Some(AppVariant::Keyboard)
    );
    let json = serde_json::to_string(&state)?;
    assert!(json.contains("\"keyboard\""));
    assert_eq!(
        serde_json::from_str::<GameState<AppChallenge>>(&json)?,
        state
    );

    // In a family the variant is stored by itself, without a branch marker, in YAML too.
    type Family = ChallengeFamily<AppChallenge, Quiz>;
    let mut entry: ChallengeConfig<ChallengeFamilyVariant<AppVariant, ()>> =
        config("scale-run", "scale", 3);
    entry.variant = Some(ChallengeFamilyVariant::First(AppVariant::Staff));
    let yaml = serde_yaml::to_string(&entry)?;
    assert!(yaml.contains("variant: staff"));
    assert_eq!(
        serde_yaml::from_str::<ChallengeConfig<ChallengeFamilyVariant<AppVariant, ()>>>(&yaml)?,
        entry
    );

    let game = Game::<Family> {
        game_paths: vec![GamePath {
            id: "mixed".into(),
            name: "Mixed".into(),
            challenges: vec![entry],
            map: None,
        }],
        challenge_factory: ChallengeFactory {
            challenge_types: vec![ChallengeFamily::First(melody("scale", &[60, 62, 64]))],
        },
        challenge_history: Default::default(),
        xp: 0,
    };
    let state = GameState::new(game);
    assert_eq!(
        state.challenge.challenge_config.variant,
        Some(ChallengeFamilyVariant::First(AppVariant::Staff))
    );
    Ok(())
}
