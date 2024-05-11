use crate::BddWorld;
use cucumber::when;
use konnektoren_core::commands::{
    game_commands::{NextChallengeCommand, PreviousChallengeCommand},
    GameCommand,
};

#[when(expr = "the next challenge is requested")]
async fn the_next_challenge_is_requested(world: &mut BddWorld) {
    let command = NextChallengeCommand();
    let state = &mut world.session.game_state;
    let result = command.execute(state);
    assert!(result.is_ok());
}

#[when(expr = "the previous challenge is requested")]
async fn the_previous_challenge_is_requested(world: &mut BddWorld) {
    let command = PreviousChallengeCommand();
    let state = &mut world.session.game_state;
    let result = command.execute(state);
    assert!(result.is_ok());
}
