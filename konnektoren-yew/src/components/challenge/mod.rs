use konnektoren_core::prelude::*;
use yew::prelude::*;

mod multiple_choice;
mod options;
mod question;

pub use multiple_choice::MultipleChoiceComponent;
pub use options::OptionsComponent;
pub use question::QuestionComponent;

#[derive(Properties, PartialEq)]
pub struct ChallengeComponentProps {
    pub challenge: Challenge,
}

#[function_component(ChallengeComponent)]
pub fn challenge_component(props: &ChallengeComponentProps) -> Html {
    let challenge_component = match &props.challenge.challenge_type {
        ChallengeType::MultipleChoice(challenge) => html! {
            <MultipleChoiceComponent challenge={challenge.clone()} />
        },
    };

    html! {
        <div class="challenge">
            <h2>{&props.challenge.challenge_type.name()}</h2>
            <p>{format!("Tasks: {}", props.challenge.challenge_config.tasks)}</p>
            <p>{format!("Unlock Points: {}", props.challenge.challenge_config.unlock_points)}</p>
            {challenge_component}
        </div>
    }
}
