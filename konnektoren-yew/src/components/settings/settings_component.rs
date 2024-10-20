use crate::components::settings::sound_config::SoundConfig;
use crate::components::MusicConfig;
use crate::model::Settings;
use crate::providers::use_settings_repository;
use crate::repository::SETTINGS_STORAGE_KEY;
use yew::prelude::*;

#[function_component(SettingsComponent)]
pub fn settings_component() -> Html {
    let settings_repository = use_settings_repository();
    let settings = use_state(|| Settings::default());
    let initial_settings = use_state(|| Settings::default());

    {
        let settings = settings.clone();
        let initial_settings = initial_settings.clone();
        let settings_repository = settings_repository.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(Some(loaded_settings)) =
                    settings_repository.get_settings(SETTINGS_STORAGE_KEY).await
                {
                    settings.set(loaded_settings.clone());
                    initial_settings.set(loaded_settings);
                }
            });
            || ()
        });
    }

    let on_change = {
        let settings = settings.clone();
        let settings_repository = settings_repository.clone();
        Callback::from(move |new_settings: Settings| {
            let settings_repository = settings_repository.clone();
            settings.set(new_settings.clone());
            wasm_bindgen_futures::spawn_local(async move {
                let _ = settings_repository
                    .save_settings(SETTINGS_STORAGE_KEY, &new_settings)
                    .await;
            });
        })
    };

    let on_save = {
        let settings = settings.clone();
        let initial_settings = initial_settings.clone();
        Callback::from(move |_| {
            initial_settings.set((*settings).clone());
        })
    };

    let has_changes = {
        let settings = (*settings).clone();
        let new_settings = (*initial_settings).clone();
        move || settings != new_settings
    };

    html! {
        <div class="settings-component">
        <h2>{ "Settings" }</h2>

        <MusicConfig settings={(*settings).clone()} on_change={on_change.clone()} />
        <SoundConfig settings={(*settings).clone()} on_change={on_change.clone()} />
        <button onclick={on_save} disabled={!has_changes()}>{ "Save" }</button>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(SettingsComponent, (),);
}
