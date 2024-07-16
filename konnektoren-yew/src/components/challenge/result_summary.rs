use crate::components::challenge::ChallengeResult;
use konnektoren_core::challenges::{Challenge, Performance};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ResultSummaryComponentProps {
    pub challenge: Challenge,
    pub challenge_result: ChallengeResult,
}

#[function_component(ResultSummaryComponent)]
pub fn result_summary_component(props: &ResultSummaryComponentProps) -> Html {
    let performance = props.challenge.performance(&props.challenge_result);

    let congratulation = if performance > 50 {
        html! {
            <p>{"Congratulations! You have completed the challenge perfectly."}</p>
        }
    } else {
        html! {}
    };

    html! {
        <div class="result-summary">
            <h2>{"Challenge Result"}</h2>
            {congratulation}
            <p>{format!("Performance: {}", performance)}</p>
        </div>
    }
}
