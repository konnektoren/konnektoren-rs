use konnektoren_core::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengeComponentProps {
    pub challenge: Challenge,
}

#[function_component(ChallengeComponent)]
pub fn challenge_component(props: &ChallengeComponentProps) -> Html {
    html! {
        <div class="challenge">
            <h2>{&props.challenge.challenge_type.name()}</h2>
            <p>{format!("Tasks: {}", props.challenge.challenge_config.tasks)}</p>
            <p>{format!("Unlock Points: {}", props.challenge.challenge_config.unlock_points)}</p>
        </div>
    }
}
