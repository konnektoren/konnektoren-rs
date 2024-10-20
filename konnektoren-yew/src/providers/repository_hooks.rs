use crate::model::{Inbox, Settings};
use crate::providers::RepositoryContext;
use crate::repository::{
    CertificateRepositoryTrait, InboxRepositoryTrait, ProfileRepositoryTrait,
    SessionRepositoryTrait, SettingsRepositoryTrait,
};
use konnektoren_core::certificates::CertificateData;
use konnektoren_core::prelude::{PlayerProfile, Session};
use std::sync::{Arc, RwLock};
use yew::prelude::*;
use yew_hooks::use_effect_once;

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
pub fn use_inbox_repository() -> Arc<dyn InboxRepositoryTrait> {
    use_context::<RepositoryContext>()
        .expect("RepositoryContext not found")
        .inbox_repository
}

#[hook]
pub fn use_session_repository() -> Arc<dyn SessionRepositoryTrait> {
    use_context::<RepositoryContext>()
        .expect("RepositoryContext not found")
        .session_repository
}

#[hook]
pub fn use_session() -> Arc<RwLock<Session>> {
    let repository_context =
        use_context::<RepositoryContext>().expect("RepositoryContext not found");
    let session = repository_context.session.clone();

    use_effect_once(move || {
        repository_context.load_session();
        || {}
    });

    session
}

#[hook]
pub fn use_profile() -> Arc<RwLock<PlayerProfile>> {
    let repository_context =
        use_context::<RepositoryContext>().expect("RepositoryContext not found");
    let profile = repository_context.profile.clone();

    use_effect_once(move || {
        repository_context.load_profile();
        || {}
    });

    profile
}

#[hook]
pub fn use_inbox() -> Arc<RwLock<Inbox>> {
    let repository_context =
        use_context::<RepositoryContext>().expect("RepositoryContext not found");
    let inbox = repository_context.inbox.clone();

    use_effect_once(move || {
        repository_context.load_inbox();
        || {}
    });

    inbox
}

#[hook]
pub fn use_settings() -> Arc<RwLock<Settings>> {
    let repository_context =
        use_context::<RepositoryContext>().expect("RepositoryContext not found");
    let settings = repository_context.settings.clone();

    use_effect_once(move || {
        repository_context.load_settings();
        || {}
    });

    settings
}

#[hook]
pub fn use_certificates() -> Arc<RwLock<Vec<CertificateData>>> {
    let repository_context =
        use_context::<RepositoryContext>().expect("RepositoryContext not found");
    let certificates = repository_context.certificates.clone();

    use_effect_once(move || {
        repository_context.load_certificates();
        || {}
    });
    certificates
}
