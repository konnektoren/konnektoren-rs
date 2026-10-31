//! A family of core's built-in challenges and an app's own is saved exactly like the
//! built-ins alone: no branch marker, existing saves load, and the app's challenges sit
//! beside them under their own names.

use konnektoren_rs::core::challenges::error::{ChallengeError, Result};
use konnektoren_rs::core::challenges::task_pattern::TaskPattern;
use konnektoren_rs::core::challenges::{
    Attempt, ChallengeDefinition, ChallengeFamily, ChallengeFamilyInput, ChallengeFamilyResult,
    ChallengeInput, ChallengeResult, ChallengeType, ChallengeVariant, Includes,
    MultipleChoiceOption, Playable, SerializedNames,
};
use konnektoren_rs::core::commands::{ChallengeCommand, Command, CommandTrait, GameCommand};
use konnektoren_rs::core::game::GameState;
use konnektoren_rs::core::session::Session;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Melody {
    id: String,
    notes: Vec<u8>,
}

impl ChallengeDefinition for Melody {
    type Result = Vec<u8>;

    fn id(&self) -> &str {
        &self.id
    }

    fn of_tasks(&self, tasks: &TaskPattern) -> Self {
        Melody {
            id: self.id.clone(),
            notes: tasks.select_items(&self.notes),
        }
    }
}

impl Playable for Melody {
    type Input = u8;
    type Variant = ChallengeVariant;

    fn result_len(result: &Vec<u8>) -> usize {
        result.len()
    }

    fn default_input(&self, _task_index: usize) -> Result<Option<u8>> {
        Ok(Some(0))
    }

    fn record_input(&self, result: &mut Option<Vec<u8>>, input: u8) -> Result<()> {
        result.get_or_insert_with(Vec::new).push(input);
        Ok(())
    }

    fn try_performance(&self, played: &Vec<u8>) -> Result<u32> {
        let correct = self
            .notes
            .iter()
            .zip(played)
            .filter(|(a, b)| a == b)
            .count();
        Ok((100 * correct / self.notes.len().max(1)) as u32)
    }

    fn solve_task(challenge: &mut Attempt<Self>, input: u8, task_index: usize) -> Result<bool> {
        let expected = *challenge
            .challenge_type
            .notes
            .get(task_index)
            .ok_or_else(|| ChallengeError::InvalidInput(format!("no task {task_index}")))?;
        let played = challenge.challenge_result.get_or_insert_with(Vec::new);
        if task_index < played.len() {
            played[task_index] = input;
        } else {
            played.push(input);
        }
        Ok(input == expected)
    }
}

/// A second app challenge, to show members of different kinds side by side.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Quiz {
    id: String,
    answer: bool,
}

impl ChallengeDefinition for Quiz {
    type Result = Vec<bool>;

    fn id(&self) -> &str {
        &self.id
    }

    fn of_tasks(&self, _tasks: &TaskPattern) -> Self {
        self.clone()
    }
}

impl Playable for Quiz {
    type Input = bool;
    type Variant = ChallengeVariant;

    fn result_len(result: &Vec<bool>) -> usize {
        result.len()
    }

    fn default_input(&self, _task_index: usize) -> Result<Option<bool>> {
        Ok(None)
    }

    fn record_input(&self, result: &mut Option<Vec<bool>>, input: bool) -> Result<()> {
        result.get_or_insert_with(Vec::new).push(input);
        Ok(())
    }

    fn try_performance(&self, given: &Vec<bool>) -> Result<u32> {
        Ok(if given.first() == Some(&self.answer) {
            100
        } else {
            0
        })
    }

    fn solve_task(challenge: &mut Attempt<Self>, input: bool, _task_index: usize) -> Result<bool> {
        challenge
            .challenge_result
            .get_or_insert_with(Vec::new)
            .push(input);
        Ok(input == challenge.challenge_type.answer)
    }
}

// The app's own challenges as one saved type, with stable names.
konnektoren_rs::core::challenge_family! {
    enum Own, results OwnAnswers, inputs OwnInput, variant = ChallengeVariant {
        Melody(Melody) = "melody",
        Quiz(Quiz) = "quiz",
    }
}

type App = ChallengeFamily<ChallengeType, Own>;
type Saved = GameState<App>;

/// Only the part of a saved session that these tests are about.
#[derive(Serialize, Deserialize)]
struct SavedSession<S> {
    game_state: S,
}

#[test]
fn legacy_game_state_loads_through_the_family_and_saves_as_core_does()
-> std::result::Result<(), Box<dyn std::error::Error>> {
    let json = include_str!("../../konnektoren-core/tests/fixtures/legacy-session.json");
    let yaml = include_str!("../../konnektoren-core/tests/fixtures/legacy-session.yml");
    let msgpack = include_bytes!("../../konnektoren-core/tests/fixtures/legacy-session.msgpack");

    let app: SavedSession<Saved> = serde_json::from_str(json)?;
    let core: SavedSession<GameState> = serde_json::from_str(json)?;
    assert_eq!(serde_json::to_value(&app)?, serde_json::to_value(&core)?);
    assert_eq!(app.game_state.game.xp, core.game_state.game.xp);
    assert!(!app.game_state.game.challenge_history.is_empty());

    let app: SavedSession<Saved> = serde_yaml::from_str(yaml)?;
    let core: SavedSession<GameState> = serde_yaml::from_str(yaml)?;
    assert_eq!(serde_yaml::to_string(&app)?, serde_yaml::to_string(&core)?);

    let app: SavedSession<Saved> = rmp_serde::from_slice(msgpack)?;
    let core: SavedSession<GameState> = rmp_serde::from_slice(msgpack)?;
    assert_eq!(
        rmp_serde::to_vec_named(&app)?,
        rmp_serde::to_vec_named(&core)?
    );

    // The whole session, with its player profile, too.
    let session: Session<App> = serde_json::from_str(json)?;
    let core: Session = serde_json::from_str(json)?;
    assert_eq!(
        serde_json::to_value(&session)?,
        serde_json::to_value(&core)?
    );
    Ok(())
}

#[test]
fn every_built_in_type_saves_identically_through_the_family()
-> std::result::Result<(), Box<dyn std::error::Error>> {
    let core = GameState::default();
    let original = serde_json::to_value(&core)?;
    let app: Saved = serde_json::from_value(original.clone())?;
    assert_eq!(serde_json::to_value(&app)?, original);
    let yaml = serde_yaml::to_string(&core)?;
    let app: Saved = serde_yaml::from_str(&yaml)?;
    assert_eq!(serde_yaml::to_string(&app)?, yaml);
    let named = rmp_serde::to_vec_named(&core)?;
    let app: Saved = rmp_serde::from_slice(&named)?;
    assert_eq!(rmp_serde::to_vec_named(&app)?, named);

    // Every built-in variant on its own, including those the default assets lack.
    for content in ChallengeType::iter() {
        let saved = serde_json::to_value(&content)?;
        let family: App = serde_json::from_value(saved.clone())?;
        assert_eq!(family, ChallengeFamily::First(content.clone()));
        assert_eq!(serde_json::to_value(&family)?, saved);
        let yaml = serde_yaml::to_string(&content)?;
        assert_eq!(serde_yaml::from_str::<App>(&yaml)?, family);

        let result = content.empty_result();
        let saved = serde_json::to_value(&result)?;
        let answers: ChallengeFamilyResult<ChallengeResult, OwnAnswers> =
            serde_json::from_value(saved.clone())?;
        assert_eq!(answers, ChallengeFamilyResult::First(result.clone()));
        assert_eq!(serde_json::to_value(&answers)?, saved);
        let yaml = serde_yaml::to_string(&result)?;
        assert_eq!(
            serde_yaml::from_str::<ChallengeFamilyResult<ChallengeResult, OwnAnswers>>(&yaml)?,
            answers
        );
    }
    Ok(())
}

#[test]
fn family_plays_both_branches_and_saves_without_branch_markers()
-> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut state: Saved = serde_json::from_value(serde_json::to_value(GameState::default())?)?;

    // A built-in challenge, driven by a command made for the built-in type alone.
    let built_in = App::first_embedding();
    let before = state.game.xp;
    Command::<ChallengeType>::Challenge(ChallengeCommand::SolveOption(0))
        .embed(&built_in)
        .execute(&mut state)?;
    Command::Game(GameCommand::NextChallenge).execute(&mut state)?;
    assert_eq!(state.game.xp, before);

    // The app's own challenge in the same state.
    let mut attempt = state.challenge.clone();
    attempt.challenge_type = ChallengeFamily::Second(Own::Melody(Melody {
        id: "scale".into(),
        notes: vec![60, 62, 64],
    }));
    attempt.challenge_result = None;
    state.challenge = attempt;
    state.current_task_index = 0;
    Command::Challenge(ChallengeCommand::Solve(ChallengeFamilyInput::Second(
        OwnInput::Melody(60),
    )))
    .execute(&mut state)?;
    assert_eq!(
        state.challenge.challenge_result,
        Some(ChallengeFamilyResult::Second(OwnAnswers::Melody(vec![60])))
    );
    // A command made for the melody type alone, embedded through the generated `Includes`.
    let melody = <Own as Includes<Melody>>::embedding().then(&App::second_embedding());
    Command::<Melody>::Challenge(ChallengeCommand::Solve(62))
        .embed(&melody)
        .execute(&mut state)?;
    assert_eq!(
        state.challenge.challenge_result,
        Some(ChallengeFamilyResult::Second(OwnAnswers::Melody(vec![
            60, 62
        ])))
    );
    // And the melody can be pulled back out of the larger attempt.
    let extracted = melody.extract_attempt(&state.challenge).unwrap();
    assert_eq!(extracted.challenge_result, Some(vec![60, 62]));
    // A member that is not the current one is not found.
    let quiz = <Own as Includes<Quiz>>::embedding().then(&App::second_embedding());
    assert!(quiz.extract_attempt(&state.challenge).is_none());
    // Input for the other branch is an explicit error.
    assert!(
        Command::Challenge(ChallengeCommand::Solve(ChallengeFamilyInput::First(
            ChallengeInput::MultipleChoice(MultipleChoiceOption::default())
        )))
        .execute(&mut state)
        .is_err()
    );

    let json = serde_json::to_string(&state)?;
    assert!(json.contains("\"melody\""));
    for marker in ["First", "Second", "branch"] {
        assert!(!json.contains(marker), "{marker}");
    }
    assert_eq!(serde_json::from_str::<Saved>(&json)?, state);
    let yaml = serde_yaml::to_string(&state)?;
    assert!(yaml.contains("!melody"));
    assert!(!yaml.contains("First") && !yaml.contains("Second"));
    assert_eq!(serde_yaml::from_str::<Saved>(&yaml)?, state);
    let named = rmp_serde::to_vec_named(&state)?;
    assert_eq!(rmp_serde::from_slice::<Saved>(&named)?, state);
    Ok(())
}

#[test]
fn names_must_differ_between_included_types_and_unknown_names_fail() {
    assert!(App::duplicate_names().is_empty());
    assert_eq!(Own::serialized_names(), ["melody", "quiz"]);
    // Including a type twice is caught before it can silently pick the first branch.
    let duplicates = ChallengeFamily::<ChallengeType, ChallengeType>::duplicate_names();
    assert!(duplicates.contains(&"multiple-choice"));

    let error = serde_json::from_str::<App>("{\"no-such-challenge\": {}}").unwrap_err();
    assert!(error.to_string().contains("no-such-challenge"), "{error}");
    assert!(serde_json::from_str::<App>("\"melody\"").is_err());
}

#[test]
fn declared_family_saves_each_member_under_its_tag_and_delegates() {
    let quiz = Own::Quiz(Quiz {
        id: "q".into(),
        answer: true,
    });
    assert_eq!(
        serde_json::to_string(&quiz).unwrap(),
        r#"{"quiz":{"id":"q","answer":true}}"#
    );
    assert!(serde_yaml::to_string(&quiz).unwrap().starts_with("!quiz"));
    assert_eq!(
        serde_json::to_string(&OwnAnswers::Quiz(vec![true])).unwrap(),
        r#"{"quiz":[true]}"#
    );
    assert_eq!(quiz.id(), "q");
    // Results of the other member are rejected, never scored as zero.
    assert_eq!(quiz.try_performance(&OwnAnswers::Quiz(vec![true])), Ok(100));
    assert_eq!(
        quiz.try_performance(&OwnAnswers::Melody(vec![60])),
        Err(ChallengeError::FamilyMismatch)
    );
    // The quiz skips unanswered tasks, the melody pads them: each member decides.
    assert_eq!(quiz.default_input(0), Ok(None));
    let melody = Own::Melody(Melody {
        id: "m".into(),
        notes: vec![60],
    });
    assert_eq!(melody.default_input(0), Ok(Some(OwnInput::Melody(0))));
}
