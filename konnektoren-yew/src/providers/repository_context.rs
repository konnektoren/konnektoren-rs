use super::RepositoryConfig;
use crate::model::{Inbox, SessionInitializer, Settings};
use crate::repository::{
    CertificateRepositoryTrait, InboxRepositoryTrait, ProfileRepositoryTrait,
    SessionRepositoryTrait, SettingsRepositoryTrait, CERTIFICATE_STORAGE_KEY, INBOX_STORAGE_KEY,
    PROFILE_STORAGE_KEY, SESSION_STORAGE_KEY, SETTINGS_STORAGE_KEY,
};
use konnektoren_core::certificates::CertificateData;
use konnektoren_core::prelude::{PlayerProfile, Session};
use std::sync::Arc;

#[derive(Clone)]
pub struct RepositoryContext {
    pub certificate_repository: Arc<dyn CertificateRepositoryTrait>,
    pub settings_repository: Arc<dyn SettingsRepositoryTrait>,
    pub profile_repository: Arc<dyn ProfileRepositoryTrait>,
    pub inbox_repository: Arc<dyn InboxRepositoryTrait>,
    pub session_repository: Arc<dyn SessionRepositoryTrait>,
    pub session_initializer: Arc<dyn SessionInitializer>,
    pub session: Arc<Session>,
    pub profile: Arc<PlayerProfile>,
    pub inbox: Arc<Inbox>,
    pub settings: Arc<Settings>,
    pub certificates: Arc<Vec<CertificateData>>,
}

impl PartialEq for RepositoryContext {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.certificate_repository, &other.certificate_repository)
            && Arc::ptr_eq(&self.settings_repository, &other.settings_repository)
            && Arc::ptr_eq(&self.profile_repository, &other.profile_repository)
            && Arc::ptr_eq(&self.inbox_repository, &other.inbox_repository)
            && Arc::ptr_eq(&self.session_repository, &other.session_repository)
            && Arc::ptr_eq(&self.session_initializer, &other.session_initializer)
            && Arc::ptr_eq(&self.session, &other.session)
            && Arc::ptr_eq(&self.profile, &other.profile)
            && Arc::ptr_eq(&self.inbox, &other.inbox)
            && Arc::ptr_eq(&self.settings, &other.settings)
            && Arc::ptr_eq(&self.certificates, &other.certificates)
    }
}

impl RepositoryContext {
    pub fn new(config: RepositoryConfig) -> Self {
        let session = config
            .session_initializer
            .initialize(&Session::default())
            .unwrap();
        Self {
            certificate_repository: config.certificate_repository,
            settings_repository: config.settings_repository,
            profile_repository: config.profile_repository,
            inbox_repository: config.inbox_repository,
            session_repository: config.session_repository,
            session_initializer: config.session_initializer,
            session: Arc::new(session),
            profile: Arc::new(PlayerProfile::default()),
            inbox: Arc::new(Inbox::default()),
            settings: Arc::new(Settings::default()),
            certificates: Arc::new(Vec::new()),
        }
    }

    pub fn load_session(&self) {
        let session_repository = self.session_repository.clone();
        let mut session = Arc::clone(&self.session);
        let session_initializer = self.session_initializer.clone();

        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(Some(loaded_session)) =
                session_repository.get_session(SESSION_STORAGE_KEY).await
            {
                let initialized_session = session_initializer.initialize(&loaded_session).unwrap();
                let session_guard = Arc::make_mut(&mut session);
                *session_guard = initialized_session;
            }
        });
    }

    pub fn load_profile(&self) {
        let profile_repository = self.profile_repository.clone();
        let mut profile = Arc::clone(&self.profile);

        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(Some(loaded_profile)) =
                profile_repository.get_profile(PROFILE_STORAGE_KEY).await
            {
                let profile_guard = Arc::make_mut(&mut profile);
                *profile_guard = loaded_profile;
            }
        });
    }

    pub fn load_inbox(&self) {
        let inbox_repository = self.inbox_repository.clone();
        let mut inbox = Arc::clone(&self.inbox);

        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(Some(loaded_inbox)) = inbox_repository.get_inbox(INBOX_STORAGE_KEY).await {
                let inbox_guard = Arc::make_mut(&mut inbox);
                *inbox_guard = loaded_inbox;
            }
        });
    }

    pub fn load_settings(&self) {
        let settings_repository = self.settings_repository.clone();
        let mut settings = Arc::clone(&self.settings);

        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(Some(loaded_settings)) =
                settings_repository.get_settings(SETTINGS_STORAGE_KEY).await
            {
                let settings_guard = Arc::make_mut(&mut settings);
                *settings_guard = loaded_settings;
            }
        });
    }

    pub fn load_certificates(&self) {
        let certificate_repository = self.certificate_repository.clone();
        let mut certificates = Arc::clone(&self.certificates);

        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(Some(loaded_certificates)) = certificate_repository
                .get_certificates(CERTIFICATE_STORAGE_KEY)
                .await
            {
                let certificates_guard = Arc::make_mut(&mut certificates);
                *certificates_guard = loaded_certificates;
            }
        });
    }
}
