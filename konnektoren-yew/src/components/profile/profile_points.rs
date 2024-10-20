use crate::providers::use_profile_repository;
use crate::repository::PROFILE_STORAGE_KEY;
use konnektoren_core::prelude::PlayerProfile;
use yew::prelude::*;

#[function_component(ProfilePointsComponent)]
pub fn profile_points_component() -> Html {
    let profile_repository = use_profile_repository();
    let profile = use_state(|| PlayerProfile::default());

    {
        let profile = profile.clone();
        let profile_repository = profile_repository.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(Some(loaded_profile)) =
                    profile_repository.get_profile(PROFILE_STORAGE_KEY).await
                {
                    profile.set(loaded_profile);
                }
            });
            || ()
        });
    }

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
