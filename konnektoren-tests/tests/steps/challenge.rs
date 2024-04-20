use crate::BddWorld;
use konnektoren_core::prelude::*;

use cucumber::{given, then, when};

#[given(expr = "default challenge is loaded")]
async fn default_challenge_is_loaded(world: &mut BddWorld) {
    let default_challenge = ChallengeType::default();
    world.challenge = default_challenge;
}

#[then(expr = "it should be a MultipleChoice challenge named {string}")]
async fn it_should_be_a_multiple_choice_challenge_named(world: &mut BddWorld, name: String) {
    match &world.challenge {
        ChallengeType::MultipleChoice(dataset) => {
            assert_eq!(dataset.name, name);
        }
    }
}

#[then(expr = "it should have exactly {int} options")]
async fn it_should_have_exactly_options(world: &mut BddWorld, options: usize) {
    match &world.challenge {
        ChallengeType::MultipleChoice(dataset) => {
            assert_eq!(dataset.options.len(), options);
        }
    }
}

#[then(expr = "it should have at least {int} questions")]
async fn it_should_have_at_least_questions(world: &mut BddWorld, questions: usize) {
    match &world.challenge {
        ChallengeType::MultipleChoice(dataset) => {
            assert!(dataset.questions.len() >= questions);
        }
    }
}

#[given(expr = "the challenge factory is initialized")]
async fn the_challenge_factory_is_initialized(world: &mut BddWorld) {
    let factory = ChallengeFactory::new();
    world.factory = Some(factory);
}

#[given(expr = "a default challenge is loaded to the factory")]
async fn a_default_challenge_is_loaded_to_the_factory(world: &mut BddWorld) {
    let challenge = ChallengeType::default();
    let factory = world.factory.as_mut().unwrap();
    factory.challenge_types.push(challenge);
}

#[when(regex = "a challenge of (.*) is created with (\\d+) questions")]
async fn a_challenge_of_is_created_with_questions(
    world: &mut BddWorld,
    challenge: String,
    questions: usize,
) {
    let challenge_config = ChallengeConfig {
        challenge: challenge.replace("\"", ""),
        questions,
        ..Default::default()
    };
    let factory = world.factory.as_ref().unwrap();
    let challenge = factory.create_challenge(&challenge_config);
    world.challenge = challenge.challenge_type.clone();
}

#[then(expr = "the challenge should have exactly {int} questions")]
async fn the_challenge_should_have_exactly_questions(world: &mut BddWorld, questions: usize) {
    match &world.challenge {
        ChallengeType::MultipleChoice(dataset) => {
            assert_eq!(dataset.questions.len(), questions);
        }
    }
}

#[then(expr = "the challenge be identified as {string}")]
async fn the_challenge_be_identified_as(world: &mut BddWorld, id: String) {
    match &world.challenge {
        ChallengeType::MultipleChoice(dataset) => {
            assert_eq!(dataset.id, id);
        }
    }
}
