use crate::BddWorld;
use konnektoren_core::prelude::*;

use cucumber::{given, then};

#[given(expr = "A new Session with id {string}")]
async fn a_new_session_with_id(world: &mut BddWorld, id: String) {
    world.session = Session::new(id);
}

#[then(expr = "player profile name is {string}")]
async fn player_profile_name_is(world: &mut BddWorld, name: String) {
    assert_eq!(world.session.player_profile.name, name);
}

#[then(expr = "player profile id is {string}")]
async fn player_profile_id_is(world: &mut BddWorld, id: String) {
    assert_eq!(world.session.player_profile.id, id);
}
