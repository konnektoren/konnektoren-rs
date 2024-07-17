use crate::storage::{ProfileStorage, Storage};
use konnektoren_core::prelude::PlayerProfile;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(ProfileConfigComponent)]
pub fn profile_config_component() -> Html {
    let profile = use_state(|| ProfileStorage::default().get("").unwrap_or_default());
    let name = use_state(|| profile.name.clone());

    let on_name_change = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            name.set(input.value());
        })
    };

    let on_save = {
        let name = name.clone();
        let profile = profile.clone();
        Callback::from(move |_| {
            let mut updated_profile: PlayerProfile = (*profile).clone();
            updated_profile.name.clone_from(&*name);
            ProfileStorage::default().update(updated_profile.clone());
            profile.set(updated_profile);
        })
    };

    let has_changes = {
        let name = name.clone();
        let initial_name = profile.name.clone();
        move || *name != initial_name
    };

    html! {
        <div class="profile-config">
            <h2>{ "Player Profile" }</h2>

            <label for="name">{ "Name" }</label>
            <input id="name" type="text" value={(*name).clone()} oninput={on_name_change} />

            <button onclick={on_save} disabled={!has_changes()}>{ "Save" }</button>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(ProfileConfigComponent, (),);
}
