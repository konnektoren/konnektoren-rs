use konnektoren_core::challenges::{ChallengeResult, MultipleChoice};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MultipleChoiceResultComponentProps {
    pub challenge: MultipleChoice,
    pub challenge_result: ChallengeResult,
}

#[function_component(MultipleChoiceResultComponent)]
pub fn multiple_choice_result_component(props: &MultipleChoiceResultComponentProps) -> Html {
    let results = match &props.challenge_result {
        ChallengeResult::MultipleChoice(options) => {
            props.challenge.questions.iter().zip(options.iter()).fold(
                Vec::new(),
                |mut results, (question, option)| {
                    let correct = if question.option == option.id {
                        "Correct"
                    } else {
                        "Incorrect"
                    };
                    results.push(format!(
                        " {}: {} - {}",
                        question.question, option.name, correct
                    ));
                    results
                },
            )
        }
    };

    html! {
        <div class="challenge-result">
            <h2>{"Challenge Result"}</h2>
            <ul>
                {for results.iter().map(|result| html! { <li>{result}</li> })}
            </ul>
        </div>
    }
}
