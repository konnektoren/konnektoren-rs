use crate::components::ChallengeRatingComponent;
use crate::i18n::use_i18n;
use konnektoren_core::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub challenge_config: ChallengeConfig,
    #[prop_or_default]
    pub api_url: Option<String>,
}

#[function_component(ChallengeInfoComponent)]
pub fn challenge_info(props: &Props) -> Html {
    let i18n = use_i18n();

    let rating_component = match props.api_url {
        Some(ref api_url) => html! {
            <ChallengeRatingComponent api_url={api_url.clone()} challenge_id={props.challenge_config.id.clone()} />
        },
        None => html! {},
    };

    html! {
        <div class="challenge-info">
            <h2>{&props.challenge_config.name}</h2>
            {rating_component}
            <div class="challenge-description">
                <p>{&props.challenge_config.description}</p>
            </div>
            <div class="tasks-info">
                <p>{format!("{}: {}", i18n.t("Tasks"), props.challenge_config.tasks)}</p>
            </div>
            <div class="unlock-points-info">
                <p>{format!("{}: {}", i18n.t("Unlock Points"), props.challenge_config.unlock_points)}</p>
            </div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ChallengeInfoComponent,
        Props {
            challenge_config: ChallengeConfig {
                id: "".to_string(),
                name: "Challenge Name".to_string(),
                description: "Challenge Description".to_string(),
                challenge: "".to_string(),
                variant: None,
                tasks: 5,
                unlock_points: 10,
                position: None,
            },
            api_url: Some("https://api.example.com/reviews".to_string()),
        },
    );
}
