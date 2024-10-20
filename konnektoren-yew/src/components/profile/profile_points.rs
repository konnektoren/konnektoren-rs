use crate::providers::use_profile;
use yew::prelude::*;

#[function_component(ProfilePointsComponent)]
pub fn profile_points_component() -> Html {
    let profile = use_profile();

    let points = profile.xp;

    html! {
        <div class="profile-points">
            <div class="icon">{"⭐️"}</div>
            <div class="profile-name">{ &profile.name }</div>
            <div class="points">{ points }</div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(ProfilePointsComponent, (),);
}
