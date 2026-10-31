use konnektoren_rs::core::{
    challenges::{ChallengeFamily, ChallengeResult, ChallengeType, MultipleChoiceOption},
    commands::{ChallengeCommand, Command, CommandError, CommandTrait},
    controller::{GameController, GameControllerTrait, GameXpPlugin},
    game::Game,
    persistence::MemoryPersistence,
};
use std::sync::Arc;

#[test]
fn repeated_finish_does_not_depend_on_a_history_plugin() {
    let mut controller =
        GameController::new(Game::default(), Arc::new(MemoryPersistence::default()));
    controller.register_plugin(Arc::new(GameXpPlugin));
    let controller = controller.init();
    let result = {
        let state = controller.game_state().lock().unwrap();
        let ChallengeType::MultipleChoice(content) = &state.challenge.challenge_type else {
            panic!("expected multiple choice")
        };
        ChallengeResult::MultipleChoice(
            content
                .questions
                .iter()
                .map(|q| MultipleChoiceOption {
                    id: q.option,
                    name: String::new(),
                })
                .collect(),
        )
    };
    controller.publish_command(Command::Challenge(ChallengeCommand::Finish(Some(
        result.clone(),
    ))));
    let first = controller.game_state().lock().unwrap().game.xp;
    controller.publish_command(Command::Challenge(ChallengeCommand::Finish(Some(result))));
    let state = controller.game_state().lock().unwrap();
    assert!(first > 0);
    assert_eq!(state.game.xp, first);
    assert_eq!(state.game.challenge_history.len(), 1);
}

#[test]
fn finishing_without_a_result_is_recorded_once() {
    let mut state = konnektoren_rs::core::game::GameState::default();
    ChallengeCommand::Finish(None).execute(&mut state).unwrap();
    assert_eq!(state.game.challenge_history.len(), 1);
    assert!(ChallengeCommand::Finish(None).execute(&mut state).is_err());
    assert_eq!(state.game.challenge_history.len(), 1);
}

#[test]
fn empty_task_list_reports_no_more_tasks() {
    let mut state = konnektoren_rs::core::game::GameState::default();
    state.game.game_paths[0].challenges[0].tasks = 0.into();
    assert!(matches!(
        ChallengeCommand::NextTask.execute(&mut state),
        Err(CommandError::ChallengeError(_))
    ));
}

#[test]
fn ambiguous_challenge_families_are_not_silently_loaded() {
    type Family = ChallengeFamily<ChallengeType, ChallengeType>;
    assert!(serde_json::to_string(&Family::Second(ChallengeType::default())).is_err());
    let saved = serde_json::to_string(&ChallengeType::default()).unwrap();
    assert!(serde_json::from_str::<Family>(&saved).is_err());
    let result = konnektoren_rs::core::challenges::ChallengeFamilyResult::<
        ChallengeResult,
        ChallengeResult,
    >::Second(ChallengeResult::default());
    assert!(serde_json::to_string(&result).is_err());
    let saved = serde_json::to_string(&ChallengeResult::default()).unwrap();
    assert!(
        serde_json::from_str::<
            konnektoren_rs::core::challenges::ChallengeFamilyResult<
                ChallengeResult,
                ChallengeResult,
            >,
        >(&saved)
        .is_err()
    );
}
