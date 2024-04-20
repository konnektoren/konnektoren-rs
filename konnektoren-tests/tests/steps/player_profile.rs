use crate::BddWorld;
use konnektoren_core::prelude::*;

use cucumber::then;

#[then(expr = "player profile name is {string}")]
async fn player_profile_name_is(world: &mut BddWorld, name: String) {
    assert_eq!(world.session.player_profile.name, name);
}

#[then(expr = "player profile id is {string}")]
async fn player_profile_id_is(world: &mut BddWorld, id: String) {
    assert_eq!(world.session.player_profile.id, id);
}

#[then(expr = "player profile xp is {int}")]
async fn player_profile_xp_is(world: &mut BddWorld, xp: Xp) {
    assert_eq!(world.session.player_profile.xp, xp);
}
