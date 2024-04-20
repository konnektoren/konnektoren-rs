use crate::BddWorld;
use konnektoren_core::prelude::Challenge;

use cucumber::{given, then};

#[given(regex = "default challenge is loaded")]
async fn default_challenge_is_loaded(world: &mut BddWorld) {
    let default_challenge = Challenge::default();
    world.challenge = default_challenge;
}

#[then(expr = "it should be a MultipleChoice challenge named {string}")]
async fn it_should_be_a_multiple_choice_challenge_named(world: &mut BddWorld, name: String) {
    match &world.challenge {
        Challenge::MultipleChoice(dataset) => {
            assert_eq!(dataset.name, name);
        }
    }
}

#[then(expr = "it should have exactly {int} options")]
async fn it_should_have_exactly_options(world: &mut BddWorld, options: usize) {
    match &world.challenge {
        Challenge::MultipleChoice(dataset) => {
            assert_eq!(dataset.options.len(), options);
        }
    }
}

#[then(expr = "it should have at least {int} questions")]
async fn it_should_have_at_least_questions(world: &mut BddWorld, questions: usize) {
    match &world.challenge {
        Challenge::MultipleChoice(dataset) => {
            assert!(dataset.questions.len() >= questions);
        }
    }
}
