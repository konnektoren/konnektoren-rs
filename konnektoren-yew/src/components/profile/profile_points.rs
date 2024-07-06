use crate::storage::{ProfileStorage, Storage};
use yew::prelude::*;

#[function_component(ProfilePointsComponent)]
pub fn profile_points_component() -> Html {
    let profile = use_state(|| ProfileStorage::default().get("").unwrap_or_default());

    let points = (*profile).xp;

    html! {
        <div class="profile-points">
            <div class="icon">{"⭐️"}</div>
            <div class="points">{ points }</div>
        </div>
    }
}
