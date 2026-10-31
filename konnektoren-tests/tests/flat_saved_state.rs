//! A hand-written flat app type that includes every built-in challenge type plus one of its
//! own. It is the target a declaration macro has to generate: the saved form is identical to
//! core's, so existing saves load, and no family position ever appears in it.

use konnektoren_rs::core::challenges::error::{ChallengeError, Result};
use konnektoren_rs::core::challenges::task_pattern::TaskPattern;
use konnektoren_rs::core::challenges::{
    Attempt, ChallengeDefinition, ChallengeEmbedding, ChallengeInput, ChallengeResult,
    ChallengeType, ChallengeVariant, ContextItemChoiceAnswers, ContextualChoice, Custom,
    CustomChallengeResult, Dialog, DialogAnswer, GapFill, GapFillAnswer, Informative,
    MultipleChoice, MultipleChoiceOption, Ordering, OrderingResult, Placeholder, Playable,
    SortTable, SortTableRow, Vocabulary,
};
use konnektoren_rs::core::commands::{ChallengeCommand, Command, CommandTrait, GameCommand};
use konnektoren_rs::core::game::GameState;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Melody {
    id: String,
    notes: Vec<u8>,
}

/// Same names and shapes as `ChallengeType`, plus the app's `melody`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Content {
    MultipleChoice(MultipleChoice),
    ContextualChoice(ContextualChoice),
    GapFill(GapFill),
    SortTable(SortTable),
    Informative(Informative),
    Ordering(Ordering),
    Custom(Custom),
    Placeholder(Placeholder),
    Vocabulary(Vocabulary),
    Dialog(Dialog),
    Melody(Melody),
}

/// Same names and shapes as `ChallengeResult`, plus the app's `Melody`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum Answers {
    MultipleChoice(Vec<MultipleChoiceOption>),
    ContextualChoice(Vec<ContextItemChoiceAnswers>),
    GapFill(Vec<GapFillAnswer>),
    SortTable(Vec<SortTableRow>),
    Informative,
    Ordering(Vec<OrderingResult>),
    Custom(CustomChallengeResult),
    Vocabulary,
    Dialog(Vec<DialogAnswer>),
    Melody(Vec<u8>),
}

/// Inputs are never saved.
#[derive(Clone, Debug, PartialEq)]
enum Input {
    Core(ChallengeInput),
    Melody(u8),
}

fn content_of(core: ChallengeType) -> Content {
    match core {
        ChallengeType::MultipleChoice(c) => Content::MultipleChoice(c),
        ChallengeType::ContextualChoice(c) => Content::ContextualChoice(c),
        ChallengeType::GapFill(c) => Content::GapFill(c),
        ChallengeType::SortTable(c) => Content::SortTable(c),
        ChallengeType::Informative(c) => Content::Informative(c),
        ChallengeType::Ordering(c) => Content::Ordering(c),
        ChallengeType::Custom(c) => Content::Custom(c),
        ChallengeType::Placeholder(c) => Content::Placeholder(c),
        ChallengeType::Vocabulary(c) => Content::Vocabulary(c),
        ChallengeType::Dialog(c) => Content::Dialog(c),
    }
}

fn core_of(content: &Content) -> Option<ChallengeType> {
    Some(match content.clone() {
        Content::MultipleChoice(c) => ChallengeType::MultipleChoice(c),
        Content::ContextualChoice(c) => ChallengeType::ContextualChoice(c),
        Content::GapFill(c) => ChallengeType::GapFill(c),
        Content::SortTable(c) => ChallengeType::SortTable(c),
        Content::Informative(c) => ChallengeType::Informative(c),
        Content::Ordering(c) => ChallengeType::Ordering(c),
        Content::Custom(c) => ChallengeType::Custom(c),
        Content::Placeholder(c) => ChallengeType::Placeholder(c),
        Content::Vocabulary(c) => ChallengeType::Vocabulary(c),
        Content::Dialog(c) => ChallengeType::Dialog(c),
        Content::Melody(_) => return None,
    })
}

fn answers_of(core: ChallengeResult) -> Answers {
    match core {
        ChallengeResult::MultipleChoice(r) => Answers::MultipleChoice(r),
        ChallengeResult::ContextualChoice(r) => Answers::ContextualChoice(r),
        ChallengeResult::GapFill(r) => Answers::GapFill(r),
        ChallengeResult::SortTable(r) => Answers::SortTable(r),
        ChallengeResult::Informative => Answers::Informative,
        ChallengeResult::Ordering(r) => Answers::Ordering(r),
        ChallengeResult::Custom(r) => Answers::Custom(r),
        ChallengeResult::Vocabulary => Answers::Vocabulary,
        ChallengeResult::Dialog(r) => Answers::Dialog(r),
    }
}

fn core_answers(answers: &Answers) -> Option<ChallengeResult> {
    Some(match answers.clone() {
        Answers::MultipleChoice(r) => ChallengeResult::MultipleChoice(r),
        Answers::ContextualChoice(r) => ChallengeResult::ContextualChoice(r),
        Answers::GapFill(r) => ChallengeResult::GapFill(r),
        Answers::SortTable(r) => ChallengeResult::SortTable(r),
        Answers::Informative => ChallengeResult::Informative,
        Answers::Ordering(r) => ChallengeResult::Ordering(r),
        Answers::Custom(r) => ChallengeResult::Custom(r),
        Answers::Vocabulary => ChallengeResult::Vocabulary,
        Answers::Dialog(r) => ChallengeResult::Dialog(r),
        Answers::Melody(_) => return None,
    })
}

fn built_in() -> ChallengeEmbedding<ChallengeType, Content> {
    ChallengeEmbedding::new(content_of, answers_of, Input::Core, |variant| variant).with_extraction(
        |content| core_of(content),
        |answers| core_answers(answers),
        |variant| Some(variant.clone()),
    )
}

impl ChallengeDefinition for Content {
    type Result = Answers;

    fn id(&self) -> &str {
        match core_of(self) {
            // The id lives inside each built-in type; borrow it from the held value.
            Some(_) => match self {
                Content::MultipleChoice(c) => &c.id,
                Content::ContextualChoice(c) => &c.id,
                Content::GapFill(c) => &c.id,
                Content::SortTable(c) => &c.id,
                Content::Informative(c) => &c.id,
                Content::Ordering(c) => &c.id,
                Content::Custom(c) => &c.id,
                Content::Placeholder(c) => &c.id,
                Content::Vocabulary(c) => &c.id,
                Content::Dialog(c) => &c.id,
                Content::Melody(c) => &c.id,
            },
            None => match self {
                Content::Melody(c) => &c.id,
                _ => unreachable!(),
            },
        }
    }

    fn of_tasks(&self, tasks: &TaskPattern) -> Self {
        match core_of(self) {
            Some(core) => content_of(core.of_tasks(tasks)),
            None => match self {
                Content::Melody(m) => Content::Melody(Melody {
                    id: m.id.clone(),
                    notes: tasks.select_items(&m.notes),
                }),
                _ => unreachable!(),
            },
        }
    }
}

impl Playable for Content {
    type Input = Input;
    type Variant = ChallengeVariant;

    fn result_len(result: &Answers) -> usize {
        match core_answers(result) {
            Some(core) => ChallengeType::result_len(&core),
            None => match result {
                Answers::Melody(notes) => notes.len(),
                _ => unreachable!(),
            },
        }
    }

    fn default_input(&self, task_index: usize) -> Result<Option<Input>> {
        match core_of(self) {
            Some(core) => Ok(core.default_input(task_index)?.map(Input::Core)),
            None => Ok(Some(Input::Melody(0))),
        }
    }

    fn record_input(&self, result: &mut Option<Answers>, input: Input) -> Result<()> {
        match (core_of(self), input) {
            (Some(core), Input::Core(input)) => {
                let mut inner = match result.take() {
                    None => None,
                    Some(answers) => match core_answers(&answers) {
                        Some(core) => Some(core),
                        None => {
                            *result = Some(answers);
                            return Err(ChallengeError::FamilyMismatch);
                        }
                    },
                };
                let outcome = core.record_input(&mut inner, input);
                *result = inner.map(answers_of);
                outcome
            }
            (None, Input::Melody(note)) => match result {
                Some(Answers::Melody(notes)) => {
                    notes.push(note);
                    Ok(())
                }
                None => {
                    *result = Some(Answers::Melody(vec![note]));
                    Ok(())
                }
                Some(_) => Err(ChallengeError::FamilyMismatch),
            },
            _ => Err(ChallengeError::FamilyMismatch),
        }
    }

    fn try_performance(&self, result: &Answers) -> Result<u32> {
        match (core_of(self), core_answers(result)) {
            (Some(core), Some(answers)) => core.try_performance(&answers),
            (None, None) => {
                let (Content::Melody(melody), Answers::Melody(played)) = (self, result) else {
                    return Err(ChallengeError::FamilyMismatch);
                };
                let correct = melody
                    .notes
                    .iter()
                    .zip(played)
                    .filter(|(a, b)| a == b)
                    .count();
                Ok((100 * correct / melody.notes.len().max(1)) as u32)
            }
            _ => Err(ChallengeError::FamilyMismatch),
        }
    }

    fn solve_task(challenge: &mut Attempt<Self>, input: Input, task_index: usize) -> Result<bool> {
        match (built_in().extract_attempt(challenge), input) {
            (Some(mut inner), Input::Core(input)) => {
                let outcome = ChallengeType::solve_task(&mut inner, input, task_index);
                *challenge = built_in().attempt(inner);
                outcome
            }
            (None, Input::Melody(note)) => {
                let Content::Melody(melody) = &challenge.challenge_type else {
                    return Err(ChallengeError::FamilyMismatch);
                };
                let expected = *melody
                    .notes
                    .get(task_index)
                    .ok_or_else(|| ChallengeError::InvalidInput(format!("no task {task_index}")))?;
                match challenge
                    .challenge_result
                    .get_or_insert(Answers::Melody(vec![]))
                {
                    Answers::Melody(played) if task_index < played.len() => {
                        played[task_index] = note
                    }
                    Answers::Melody(played) => played.push(note),
                    _ => return Err(ChallengeError::FamilyMismatch),
                }
                Ok(note == expected)
            }
            _ => Err(ChallengeError::FamilyMismatch),
        }
    }

    fn option_input(&self, option_index: usize) -> Result<Input> {
        match core_of(self) {
            Some(core) => core.option_input(option_index).map(Input::Core),
            None => Err(ChallengeError::InvalidChallengeType),
        }
    }
}

type Flat = GameState<Content>;

/// Only the part of a saved session that these tests are about.
#[derive(Serialize, Deserialize)]
struct Saved<S> {
    game_state: S,
}

/// The sessions saved before this work, in every format they were stored in.
#[test]
fn legacy_game_state_loads_through_the_flat_type_and_saves_as_core_does()
-> std::result::Result<(), Box<dyn std::error::Error>> {
    let json = include_str!("../../konnektoren-core/tests/fixtures/legacy-session.json");
    let yaml = include_str!("../../konnektoren-core/tests/fixtures/legacy-session.yml");
    let msgpack = include_bytes!("../../konnektoren-core/tests/fixtures/legacy-session.msgpack");

    let flat: Saved<Flat> = serde_json::from_str(json)?;
    let core: Saved<GameState> = serde_json::from_str(json)?;
    assert_eq!(serde_json::to_value(&flat)?, serde_json::to_value(&core)?);
    assert_eq!(flat.game_state.game.xp, core.game_state.game.xp);
    assert_eq!(
        flat.game_state.game.challenge_history.len(),
        core.game_state.game.challenge_history.len()
    );
    assert!(flat.game_state.game.challenge_history.len() > 0);

    let flat: Saved<Flat> = serde_yaml::from_str(yaml)?;
    let core: Saved<GameState> = serde_yaml::from_str(yaml)?;
    assert_eq!(serde_yaml::to_string(&flat)?, serde_yaml::to_string(&core)?);

    let flat: Saved<Flat> = rmp_serde::from_slice(msgpack)?;
    let core: Saved<GameState> = rmp_serde::from_slice(msgpack)?;
    assert_eq!(
        rmp_serde::to_vec_named(&flat)?,
        rmp_serde::to_vec_named(&core)?
    );
    Ok(())
}

#[test]
fn every_built_in_type_saves_identically_through_the_flat_type()
-> std::result::Result<(), Box<dyn std::error::Error>> {
    // All default content and paths, as core saves them.
    let core = GameState::default();
    let original = serde_json::to_value(&core)?;
    let flat: Flat = serde_json::from_value(original.clone())?;
    assert_eq!(serde_json::to_value(&flat)?, original);
    let yaml = serde_yaml::to_string(&core)?;
    let flat: Flat = serde_yaml::from_str(&yaml)?;
    assert_eq!(serde_yaml::to_string(&flat)?, yaml);
    let named = rmp_serde::to_vec_named(&core)?;
    let flat: Flat = rmp_serde::from_slice(&named)?;
    assert_eq!(rmp_serde::to_vec_named(&flat)?, named);

    // And every built-in variant on its own, including those the default assets lack.
    for content in ChallengeType::iter() {
        let saved = serde_json::to_value(&content)?;
        let flat: Content = serde_json::from_value(saved.clone())?;
        assert_eq!(serde_json::to_value(&flat)?, saved, "{content:?}");
        assert_eq!(core_of(&flat), Some(content.clone()));
        let result = content.empty_result();
        let saved = serde_json::to_value(&result)?;
        let flat: Answers = serde_json::from_value(saved.clone())?;
        assert_eq!(serde_json::to_value(&flat)?, saved, "{result:?}");
    }
    Ok(())
}

#[test]
fn flat_state_plays_built_in_and_app_challenges_and_saves_without_family_markers()
-> std::result::Result<(), Box<dyn std::error::Error>> {
    // A whole default game, saved by core and loaded as the flat type.
    let mut state: Flat = serde_json::from_value(serde_json::to_value(GameState::default())?)?;

    // Built-in play through the flat type, as the default UI would drive it.
    let before = state.game.xp;
    Command::Challenge(ChallengeCommand::SolveOption(0)).execute(&mut state)?;
    Command::Game(GameCommand::NextChallenge).execute(&mut state)?;
    assert_eq!(state.game.xp, before);

    // The app's own challenge in the same state.
    let melody = Content::Melody(Melody {
        id: "scale".into(),
        notes: vec![60, 62],
    });
    let mut attempt = state.challenge.clone();
    attempt.challenge_type = melody.clone();
    attempt.challenge_result = None;
    state.challenge = attempt;
    state.current_task_index = 0;
    Command::Challenge(ChallengeCommand::Solve(Input::Melody(60))).execute(&mut state)?;
    assert_eq!(
        state.challenge.challenge_result,
        Some(Answers::Melody(vec![60]))
    );
    // A built-in input against the app's challenge is an explicit error.
    assert!(
        Command::Challenge(ChallengeCommand::Solve(Input::Core(
            ChallengeInput::MultipleChoice(MultipleChoiceOption::default())
        )))
        .execute(&mut state)
        .is_err()
    );

    let json = serde_json::to_string(&state)?;
    assert!(json.contains("\"melody\""));
    assert!(!json.contains("First") && !json.contains("Second"));
    assert!(!json.contains("branch"));
    assert_eq!(serde_json::from_str::<Flat>(&json)?, state);
    let yaml = serde_yaml::to_string(&state)?;
    assert!(yaml.contains("!melody"));
    assert_eq!(serde_yaml::from_str::<Flat>(&yaml)?, state);
    Ok(())
}

#[test]
fn whole_legacy_session_loads_as_a_flat_session_and_saves_as_core_does()
-> std::result::Result<(), Box<dyn std::error::Error>> {
    use konnektoren_rs::core::session::Session;
    let json = include_str!("../../konnektoren-core/tests/fixtures/legacy-session.json");
    let flat: Session<Content> = serde_json::from_str(json)?;
    let core: Session = serde_json::from_str(json)?;
    assert_eq!(serde_json::to_value(&flat)?, serde_json::to_value(&core)?);
    assert_eq!(flat.player_profile, core.player_profile);

    let yaml = include_str!("../../konnektoren-core/tests/fixtures/legacy-session.yml");
    let flat: Session<Content> = serde_yaml::from_str(yaml)?;
    let core: Session = serde_yaml::from_str(yaml)?;
    assert_eq!(serde_yaml::to_string(&flat)?, serde_yaml::to_string(&core)?);

    // An application starts its own session from a game state of its own type.
    let game_state: Flat = serde_json::from_value(serde_json::to_value(GameState::default())?)?;
    let fresh = Session::new_with_state("player".into(), game_state);
    assert_eq!(fresh.player_profile.id, "player");
    assert_eq!(
        serde_json::from_str::<Session<Content>>(&serde_json::to_string(&fresh)?)?,
        fresh
    );
    Ok(())
}
