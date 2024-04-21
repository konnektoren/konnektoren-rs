use konnektoren_core::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengeConfigComponentProps {
    pub challenge_config: ChallengeConfig,
}

#[function_component(ChallengeConfigComponent)]
pub fn challenge_config_component(props: &ChallengeConfigComponentProps) -> Html {
    html! {
        <div class="challenge-config" id={props.challenge_config.id.to_string()}>
            <h2>{&props.challenge_config.name}</h2>
            <p>{&props.challenge_config.description}</p>
            <p>{format!("Questions: {}", props.challenge_config.questions)}</p>
            <p>{format!("Unlock Points: {}", props.challenge_config.unlock_points)}</p>
        </div>
    }
}
