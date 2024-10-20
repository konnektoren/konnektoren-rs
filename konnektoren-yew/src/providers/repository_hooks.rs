use crate::model::Settings;
use crate::providers::RepositoryContext;
use crate::repository::{
    CertificateRepositoryTrait, ProfileRepositoryTrait, SettingsRepositoryTrait,
    CERTIFICATE_STORAGE_KEY, PROFILE_STORAGE_KEY, SETTINGS_STORAGE_KEY,
};
use konnektoren_core::certificates::CertificateData;
use konnektoren_core::prelude::PlayerProfile;
use std::sync::Arc;
use yew::prelude::*;

#[hook]
pub fn use_certificate_repository() -> Arc<dyn CertificateRepositoryTrait> {
    use_context::<RepositoryContext>()
        .expect("RepositoryContext not found")
        .certificate_repository
        .clone()
}

#[hook]
pub fn use_settings_repository() -> Arc<dyn SettingsRepositoryTrait> {
    use_context::<RepositoryContext>()
        .expect("RepositoryContext not found")
        .settings_repository
}

#[hook]
pub fn use_profile_repository() -> Arc<dyn ProfileRepositoryTrait> {
    use_context::<RepositoryContext>()
        .expect("RepositoryContext not found")
        .profile_repository
}

#[hook]
pub fn use_settings() -> UseStateHandle<Settings> {
    let settings_repository = use_settings_repository();
    let settings = use_state(|| Settings::default());

    {
        let settings = settings.clone();
        let settings_repository = settings_repository.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(Some(loaded_settings)) =
                    settings_repository.get_settings(SETTINGS_STORAGE_KEY).await
                {
                    settings.set(loaded_settings.clone());
                }
            });
            || ()
        });
    }

    settings
}

#[hook]
pub fn use_certificate() -> UseStateHandle<Option<Vec<CertificateData>>> {
    let certificate_repository = use_certificate_repository();
    let certificate = use_state(|| None);
    {
        let certificate = certificate.clone();
        let certificate_repository = certificate_repository.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(Some(loaded_certificate)) = certificate_repository
                    .get_certificates(CERTIFICATE_STORAGE_KEY)
                    .await
                {
                    certificate.set(Some(loaded_certificate));
                }
            });
            || ()
        });
    }
    certificate
}

#[hook]
pub fn use_profile() -> UseStateHandle<PlayerProfile> {
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

    profile
}
