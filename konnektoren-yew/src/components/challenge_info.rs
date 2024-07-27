use crate::i18n::use_i18n;
use konnektoren_core::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub challenge_config: ChallengeConfig,
}

#[function_component(ChallengeInfoComponent)]
pub fn challenge_info(props: &Props) -> Html {
    let i18n = use_i18n();
    html! {
        <div class="challenge-info">
            <h2>{&props.challenge_config.name}</h2>
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
            }
        },
    );
}
