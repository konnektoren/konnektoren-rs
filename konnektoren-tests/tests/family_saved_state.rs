//! A family of core's built-in challenges and an app's own is saved exactly like the
//! built-ins alone: no branch marker, existing saves load, and the app's challenges sit
//! beside them under their own names.

use konnektoren_rs::core::challenges::error::{ChallengeError, Result};
use konnektoren_rs::core::challenges::task_pattern::TaskPattern;
use konnektoren_rs::core::challenges::{
    Attempt, ChallengeDefinition, ChallengeFamily, ChallengeFamilyInput, ChallengeFamilyResult,
    ChallengeInput, ChallengeResult, ChallengeType, ChallengeVariant, MultipleChoiceOption,
    Playable, SerializedNames,
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

/// The app's own challenges. Being an enum with names is all a family needs to save it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, strum_macros::VariantNames)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
enum Own {
    Melody(Melody),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, strum_macros::VariantNames)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
enum OwnAnswers {
    Melody(Vec<u8>),
}

impl SerializedNames for Own {
    fn serialized_names() -> Vec<&'static str> {
        <Self as strum::VariantNames>::VARIANTS.to_vec()
    }
}

impl SerializedNames for OwnAnswers {
    fn serialized_names() -> Vec<&'static str> {
        <Self as strum::VariantNames>::VARIANTS.to_vec()
    }
}

impl ChallengeDefinition for Own {
    type Result = OwnAnswers;

    fn id(&self) -> &str {
        let Own::Melody(melody) = self;
        &melody.id
    }

    fn of_tasks(&self, tasks: &TaskPattern) -> Self {
        let Own::Melody(melody) = self;
        Own::Melody(Melody {
            id: melody.id.clone(),
            notes: tasks.select_items(&melody.notes),
        })
    }
}

impl Playable for Own {
    type Input = u8;
    type Variant = ChallengeVariant;

    fn result_len(result: &OwnAnswers) -> usize {
        let OwnAnswers::Melody(notes) = result;
        notes.len()
    }

    fn default_input(&self, _task_index: usize) -> Result<Option<u8>> {
        Ok(Some(0))
    }

    fn record_input(&self, result: &mut Option<OwnAnswers>, input: u8) -> Result<()> {
        let OwnAnswers::Melody(notes) = result.get_or_insert(OwnAnswers::Melody(vec![]));
        notes.push(input);
        Ok(())
    }

    fn try_performance(&self, result: &OwnAnswers) -> Result<u32> {
        let (Own::Melody(melody), OwnAnswers::Melody(played)) = (self, result);
        let correct = melody
            .notes
            .iter()
            .zip(played)
            .filter(|(a, b)| a == b)
            .count();
        Ok((100 * correct / melody.notes.len().max(1)) as u32)
    }

    fn solve_task(challenge: &mut Attempt<Self>, input: u8, task_index: usize) -> Result<bool> {
        let Own::Melody(melody) = &challenge.challenge_type;
        let expected = *melody
            .notes
            .get(task_index)
            .ok_or_else(|| ChallengeError::InvalidInput(format!("no task {task_index}")))?;
        let OwnAnswers::Melody(played) = challenge
            .challenge_result
            .get_or_insert(OwnAnswers::Melody(vec![]));
        if task_index < played.len() {
            played[task_index] = input;
        } else {
            played.push(input);
        }
        Ok(input == expected)
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
        notes: vec![60, 62],
    }));
    attempt.challenge_result = None;
    state.challenge = attempt;
    state.current_task_index = 0;
    Command::Challenge(ChallengeCommand::Solve(ChallengeFamilyInput::Second(60)))
        .execute(&mut state)?;
    assert_eq!(
        state.challenge.challenge_result,
        Some(ChallengeFamilyResult::Second(OwnAnswers::Melody(vec![60])))
    );
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
    // Including a type twice is caught before it can silently pick the first branch.
    let duplicates = ChallengeFamily::<ChallengeType, ChallengeType>::duplicate_names();
    assert!(duplicates.contains(&"multiple-choice"));

    let error = serde_json::from_str::<App>("{\"no-such-challenge\": {}}").unwrap_err();
    assert!(error.to_string().contains("no-such-challenge"), "{error}");
    assert!(serde_json::from_str::<App>("\"melody\"").is_err());
}
