use konnektoren_core::{
    challenges::{
        Challenge, ChallengeConfig, ChallengeInput, ChallengeResult, ChallengeType,
        MultipleChoiceOption, Performance, Solvable,
    },
    commands::{ChallengeCommand, Command, CommandTrait},
    persistence::{GameStatePersistence, MemoryPersistence},
    session::Session,
};
use serde::{Deserialize, Serialize};

// The pre-Option wire shape. Keep this independent of the current Challenge.
#[derive(Serialize, Deserialize)]
struct LegacyChallenge {
    challenge_type: ChallengeType,
    challenge_config: ChallengeConfig,
    challenge_result: ChallengeResult,
    start_time: Option<chrono::DateTime<chrono::Utc>>,
    end_time: Option<chrono::DateTime<chrono::Utc>>,
}

#[test]
fn existing_sessions_load_without_losing_progress() -> Result<(), Box<dyn std::error::Error>> {
    let json = include_str!("fixtures/legacy-session.json");
    let session: Session = serde_json::from_str(json)?;
    let yaml: Session = serde_yaml::from_str(include_str!("fixtures/legacy-session.yml"))?;
    let msgpack: Session =
        rmp_serde::from_slice(include_bytes!("fixtures/legacy-session.msgpack"))?;
    assert_eq!(session, yaml);
    assert_eq!(session, msgpack);
    assert_eq!(session.game_state.game.xp, 42);
    assert_eq!(session.game_state.game.challenge_history.len(), 1);
    assert!(session.game_state.challenge.start_time.is_some());
    assert!(session.game_state.challenge.end_time.is_some());
    assert!(session.game_state.challenge.challenge_result.is_some());
    // Some adds no extra wrapper to existing JSON or MessagePack results.
    assert_eq!(
        serde_json::to_value(&session)?,
        serde_json::from_str::<serde_json::Value>(json)?
    );
    assert_eq!(
        rmp_serde::from_slice::<Session>(&rmp_serde::to_vec_named(&session)?)?,
        session
    );

    let persistence = MemoryPersistence::new(session.game_state.clone());
    let mut restored = persistence.load_game_state()?;
    // Continue submitting answers after loading the pre-change save.
    restored.challenge.solve(
        ChallengeInput::MultipleChoice(MultipleChoiceOption {
            id: 1,
            name: "die".into(),
        }),
        1,
    )?;
    persistence.save_game_state(&restored)?;
    assert_eq!(persistence.load_game_state()?, restored);
    assert_eq!(
        restored
            .challenge
            .challenge_result
            .as_ref()
            .map(ChallengeResult::len),
        Some(2)
    );
    Ok(())
}

#[test]
fn every_legacy_result_loads_and_remains_readable_by_the_legacy_shape()
-> Result<(), Box<dyn std::error::Error>> {
    use strum::IntoEnumIterator;
    for content in ChallengeType::iter() {
        // Existing legacy fixtures cover answers; this covers empty/unit variants too.
        let result = match &content {
            ChallengeType::MultipleChoice(_) | ChallengeType::Placeholder(_) => {
                ChallengeResult::MultipleChoice(vec![])
            }
            ChallengeType::ContextualChoice(_) => ChallengeResult::ContextualChoice(vec![]),
            ChallengeType::GapFill(_) => ChallengeResult::GapFill(vec![]),
            ChallengeType::SortTable(_) => ChallengeResult::SortTable(vec![]),
            ChallengeType::Informative(_) => ChallengeResult::Informative,
            ChallengeType::Ordering(_) => ChallengeResult::Ordering(vec![]),
            ChallengeType::Custom(_) => ChallengeResult::Custom(Default::default()),
            ChallengeType::Vocabulary(_) => ChallengeResult::Vocabulary,
            ChallengeType::Dialog(_) => ChallengeResult::Dialog(vec![]),
        };
        let legacy = LegacyChallenge {
            challenge_type: content,
            challenge_config: ChallengeConfig::default(),
            challenge_result: result.clone(),
            start_time: None,
            end_time: None,
        };
        let json: Challenge = serde_json::from_str(&serde_json::to_string(&legacy)?)?;
        let yaml: Challenge = serde_yaml::from_str(&serde_yaml::to_string(&legacy)?)?;
        let binary: Challenge = rmp_serde::from_slice(&rmp_serde::to_vec_named(&legacy)?)?;
        assert_eq!(json.challenge_result, Some(result));
        assert_eq!(json, yaml);
        assert_eq!(json, binary);
        let old_reader: LegacyChallenge = serde_json::from_str(&serde_json::to_string(&json)?)?;
        assert_eq!(Some(old_reader.challenge_result), json.challenge_result);
    }
    Ok(())
}

#[test]
fn missing_and_null_results_load_but_corrupt_results_fail() -> Result<(), Box<dyn std::error::Error>>
{
    let challenge = Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
    assert!(challenge.challenge_result.is_none());
    assert_eq!(challenge.performance(&challenge.challenge_result), 0);
    let yaml: Challenge = serde_yaml::from_str(&serde_yaml::to_string(&challenge)?)?;
    let binary: Challenge = rmp_serde::from_slice(&rmp_serde::to_vec_named(&challenge)?)?;
    assert_eq!(yaml, challenge);
    assert_eq!(binary, challenge);
    let mut value = serde_json::to_value(&challenge)?;
    assert!(serde_json::from_value::<LegacyChallenge>(value.clone()).is_err());
    let null: Challenge = serde_json::from_value(value.clone())?;
    assert!(null.challenge_result.is_none());
    value
        .as_object_mut()
        .ok_or("expected a challenge object")?
        .remove("challenge_result");
    let missing: Challenge = serde_json::from_value(value.clone())?;
    assert!(missing.challenge_result.is_none());
    value["challenge_result"] = serde_json::json!({"UnknownResult": []});
    assert!(serde_json::from_value::<Challenge>(value).is_err());
    Ok(())
}

#[test]
fn solving_and_finishing_preserve_results() -> Result<(), Box<dyn std::error::Error>> {
    let mut session: Session = serde_json::from_str(include_str!("fixtures/legacy-session.json"))?;
    let challenge = Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
    Command::Challenge(ChallengeCommand::Start(challenge)).execute(&mut session.game_state)?;
    assert!(session.game_state.challenge.challenge_result.is_none());
    session.game_state.challenge.solve(
        ChallengeInput::MultipleChoice(MultipleChoiceOption::default()),
        0,
    )?;
    let result = session.game_state.challenge.challenge_result.clone();
    Command::Challenge(ChallengeCommand::Finish(None)).execute(&mut session.game_state)?;
    assert_eq!(session.game_state.challenge.challenge_result, result);
    assert!(session.game_state.challenge.end_time.is_some());
    Ok(())
}
