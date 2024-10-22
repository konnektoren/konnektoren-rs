use crate::BddWorld;
use cucumber::{given, then, when};
use konnektoren_core::commands::{ChallengeCommand, Command, CommandTrait, GameCommand};
use konnektoren_core::game::GamePath;

#[when(expr = "the next challenge is requested")]
async fn the_next_challenge_is_requested(world: &mut BddWorld) {
    let command = Command::Game(GameCommand::NextChallenge);
    let state = &mut world.session.game_state;
    world.last_command_result = command.execute(state);
}

#[when(expr = "the previous challenge is requested")]
async fn the_previous_challenge_is_requested(world: &mut BddWorld) {
    let command = Command::Game(GameCommand::PreviousChallenge);
    let state = &mut world.session.game_state;
    world.last_command_result = command.execute(state);

    if world.session.game_state.current_challenge_index == 0 {
        assert!(world.last_command_result.is_err());
    } else {
        assert!(world.last_command_result.is_ok());
    }
}

#[given(expr = "the current challenge is the last challenge")]
async fn the_current_challenge_is_the_last_challenge(world: &mut BddWorld) {
    world.session.game_state.current_challenge_index = world.session.game_state.game.game_paths
        [world.session.game_state.current_game_path]
        .challenges
        .len()
        - 1;
}

#[given(expr = "the current task index is the last task index")]
async fn the_current_task_index_is_the_last_task_index(world: &mut BddWorld) {
    let current_game_path: &GamePath =
        &world.session.game_state.game.game_paths[world.session.game_state.current_game_path];
    let challenge_config =
        &current_game_path.challenges[world.session.game_state.current_challenge_index];
    let max_tasks = challenge_config.tasks.len();
    world.session.game_state.current_task_index = max_tasks - 1;
}

#[given(expr = "the current challenge is the first challenge")]
async fn the_current_challenge_is_the_first_challenge(world: &mut BddWorld) {
    world.session.game_state.current_challenge_index = 0;
}

#[when(expr = "the challenge is finished")]
async fn the_challenge_is_finished(world: &mut BddWorld) {
    let command = Command::Challenge(ChallengeCommand::Finish(None));
    let state = &mut world.session.game_state;
    let result = command.execute(state);
    assert!(result.is_ok());
}

#[then(expr = "an error should be raised with message {string}")]
async fn an_error_should_be_raised_with_message(world: &mut BddWorld, expected_error: String) {
    match &world.last_command_result {
        Ok(_) => panic!("Expected an error, but got Ok"),
        Err(err) => assert_eq!(err.to_string(), expected_error),
    }
}

#[when(expr = "the challenge is solved with option {int}")]
async fn the_challenge_is_solved_with_option(world: &mut BddWorld, option_index: usize) {
    let command = Command::Challenge(ChallengeCommand::SolveOption(option_index));
    let state = &mut world.session.game_state;
    command.execute(state).unwrap();
}

#[then(expr = "the current task index should be {int}")]
async fn the_current_task_index_should_be(world: &mut BddWorld, expected_index: usize) {
    assert_eq!(world.session.game_state.current_task_index, expected_index);
}

#[when(expr = "the next task is requested")]
async fn the_next_task_is_requested(world: &mut BddWorld) {
    let command = Command::Challenge(ChallengeCommand::NextTask);
    let state = &mut world.session.game_state;
    world.last_command_result = command.execute(state);
}

#[when(expr = "the previous task is requested")]
async fn the_previous_task_is_requested(world: &mut BddWorld) {
    let command = Command::Challenge(ChallengeCommand::PreviousTask);
    let state = &mut world.session.game_state;
    world.last_command_result = command.execute(state);
}

#[given(expr = "the current task index is {int}")]
async fn the_current_task_index_is(world: &mut BddWorld, index: usize) {
    world.session.game_state.current_task_index = index;
}
