use konnektoren_core::prelude::*;
use yew::prelude::*;

mod actions;
mod multiple_choice;
mod multiple_choice_result;
mod options;
mod question;

pub use actions::{ChallengeActions, ChallengeActionsComponent};
pub use multiple_choice::MultipleChoiceComponent;
pub use multiple_choice_result::MultipleChoiceResultComponent;
pub use options::OptionsComponent;
pub use question::QuestionComponent;

#[derive(Properties, PartialEq)]
pub struct ChallengeComponentProps {
    pub challenge: Challenge,
}

#[function_component(ChallengeComponent)]
pub fn challenge_component(props: &ChallengeComponentProps) -> Html {
    let challenge_result = use_state(|| Option::<ChallengeResult>::None);

    let handle_finish = {
        let challenge_result = challenge_result.clone();
        Callback::from(move |result: ChallengeResult| {
            log::info!("Challenge Result: {:?}", result);
            challenge_result.set(Some(result));
        })
    };

    let challenge_component = match (&*challenge_result, &props.challenge.challenge_type) {
        (None, ChallengeType::MultipleChoice(challenge)) => html! {
            <MultipleChoiceComponent challenge={challenge.clone()} on_finish={handle_finish} />
        },
        _ => html! {},
    };

    let challenge_result_component = match (&*challenge_result, &props.challenge.challenge_type) {
        (Some(result), ChallengeType::MultipleChoice(challenge)) => html! {
            <MultipleChoiceResultComponent challenge={challenge.clone()} challenge_result={result.clone()} />
        },
        _ => html! {},
    };

    html! {
        <div class="challenge">
            <h2>{&props.challenge.challenge_type.name()}</h2>
            <div class="tasks-info">
                <p>{format!("Tasks: {}", props.challenge.challenge_config.tasks)}</p>
            </div>
            <div class="unlock-points-info">
                <p>{format!("Unlock Points: {}", props.challenge.challenge_config.unlock_points)}</p>
            </div>
            {challenge_component}
            {challenge_result_component}
        </div>
    }
}
