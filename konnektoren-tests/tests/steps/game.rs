use crate::BddWorld;
use konnektoren_core::game::GamePath;

use cucumber::{given, then};

#[given(regex = "default game path is loaded")]
async fn default_game_path_is_loaded(world: &mut BddWorld) {
    let default_game_path = GamePath::default();
    world.game_path = default_game_path;
}

#[then(expr = "game path should be named {string}")]
async fn game_path_should_be_named(world: &mut BddWorld, name: String) {
    assert_eq!(world.game_path.name, name);
}

#[then(expr = "it should have at least {int} challenges")]
async fn it_should_have_at_least_challenges(world: &mut BddWorld, challenges: usize) {
    assert!(world.game_path.challenges.len() >= challenges);
}
