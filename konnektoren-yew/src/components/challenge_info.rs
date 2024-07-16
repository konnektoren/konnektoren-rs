use konnektoren_core::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub challenge_config: ChallengeConfig,
}

#[function_component(ChallengeInfoComponent)]
pub fn challenge_info(props: &Props) -> Html {
    html! {
        <div class="challenge-info">
            <h2>{&props.challenge_config.name}</h2>
            <div class="challenge-description">
                <p>{&props.challenge_config.description}</p>
            </div>
            <div class="tasks-info">
                <p>{format!("Tasks: {}", props.challenge_config.tasks)}</p>
            </div>
            <div class="unlock-points-info">
                <p>{format!("Unlock Points: {}", props.challenge_config.unlock_points)}</p>
            </div>
        </div>
    }
}
