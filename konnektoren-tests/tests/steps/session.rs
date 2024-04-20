use crate::BddWorld;
use konnektoren_core::prelude::*;

use cucumber::given;

#[given(expr = "A new Session with id {string}")]
async fn a_new_session_with_id(world: &mut BddWorld, id: String) {
    world.session = Session::new(id);
}
