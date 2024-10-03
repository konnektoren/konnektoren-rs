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
            <div class="challenge-info__rating">
                <ChallengeRatingComponent api_url={api_url.clone()} challenge_id={props.challenge_config.id.clone()} />
            </div>
        },
        None => html! {},
    };

    html! {
        <div class="challenge-info">
            <h2 class="challenge-info__title">{&props.challenge_config.name}</h2>
            {rating_component}
            <div class="challenge-info__description">
                <p>{&props.challenge_config.description}</p>
            </div>
            <div class="challenge-info__meta">
                <p class="challenge-info__tasks">{format!("{}: {}", i18n.t("Tasks"), props.challenge_config.tasks.len())}</p>
                <p class="challenge-info__unlock-points">{format!("{}: {}", i18n.t("Unlock Points"), props.challenge_config.unlock_points)}</p>
            </div>
        </div>
    }
}
