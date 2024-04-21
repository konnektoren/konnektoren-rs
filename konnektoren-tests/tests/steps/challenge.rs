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
    tasks: usize,
) {
    let challenge_config = ChallengeConfig {
        challenge: challenge.replace("\"", ""),
        tasks,
        ..Default::default()
    };
    let factory = world.factory.as_ref().unwrap();
    let challenge = factory.create_challenge(&challenge_config);
    world.challenge = challenge.unwrap().challenge_type.clone();
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

#[given(expr = "a multiple choice challenge is set up with a question of option {int}")]
async fn a_multiple_choice_challenge_is_set_up_with_a_question_of_option(
    world: &mut BddWorld,
    option: usize,
) {
    let id = "123".to_string();
    let name = "Test".to_string();
    let options = vec![MultipleChoiceOption {
        id: option,
        name: "Option".to_string(),
    }];
    let questions = vec![Question {
        question: "Question".to_string(),
        help: "Help 1".to_string(),
        option,
    }];
    let dataset = MultipleChoice {
        id,
        name,
        options,
        questions,
    };
    world.challenge = ChallengeType::MultipleChoice(dataset);
}

#[when(expr = "the multiple choice challenge is solved with option {int}")]
async fn the_multiple_choice_challenge_is_solved_with_option(world: &mut BddWorld, option: usize) {
    let mut challenge_result = ChallengeResult::default();
    let input = ChallengeInput::MultipleChoice(MultipleChoiceOption {
        id: option,
        name: "Option".to_string(),
    });
    let result = challenge_result.add_input(input);
    assert!(result.is_ok());
    world.challenge_result = Some(challenge_result);
}

#[then(expr = "the result performance should be at least {int}")]
async fn the_result_performance_should_be_at_least(world: &mut BddWorld, performance: i32) {
    let challenge_result = world.challenge_result.as_ref().unwrap();
    let challenge = &world.challenge;
    let score = challenge.performance(&challenge_result);
    assert!(score >= performance);
}
