use super::RepositoryConfig;
use crate::model::{Inbox, SessionInitializer, Settings};
use crate::repository::{
    CertificateRepositoryTrait, InboxRepositoryTrait, ProfileRepositoryTrait,
    SessionRepositoryTrait, SettingsRepositoryTrait, CERTIFICATE_STORAGE_KEY, INBOX_STORAGE_KEY,
    PROFILE_STORAGE_KEY, SESSION_STORAGE_KEY, SETTINGS_STORAGE_KEY,
};
use konnektoren_core::certificates::CertificateData;
use konnektoren_core::prelude::{PlayerProfile, Session};
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct RepositoryContext {
    pub certificate_repository: Arc<dyn CertificateRepositoryTrait>,
    pub settings_repository: Arc<dyn SettingsRepositoryTrait>,
    pub profile_repository: Arc<dyn ProfileRepositoryTrait>,
    pub inbox_repository: Arc<dyn InboxRepositoryTrait>,
    pub session_repository: Arc<dyn SessionRepositoryTrait>,
    pub session_initializer: Arc<dyn SessionInitializer>,
    pub session: Arc<RwLock<Session>>,
    pub inbox: Arc<RwLock<Inbox>>,
    pub settings: Arc<RwLock<Settings>>,
    pub certificates: Arc<RwLock<Vec<CertificateData>>>,
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
            session: Arc::new(RwLock::new(session)),
            inbox: Arc::new(RwLock::new(Inbox::default())),
            settings: Arc::new(RwLock::new(Settings::default())),
            certificates: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn load_session(&self) {
        let session_repository = self.session_repository.clone();
        let session = Arc::clone(&self.session);
        let session_initializer = self.session_initializer.clone();

        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(Some(loaded_session)) =
                session_repository.get_session(SESSION_STORAGE_KEY).await
            {
                let initialized_session = session_initializer.initialize(&loaded_session).unwrap();
                let mut session_guard = session.write().unwrap();
                *session_guard = initialized_session;
            }
        });
    }

    pub fn load_settings(&self) {
        let settings_repository = self.settings_repository.clone();
        let settings = Arc::clone(&self.settings);

        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(Some(loaded_settings)) =
                settings_repository.get_settings(SETTINGS_STORAGE_KEY).await
            {
                let mut settings_guard = settings.write().unwrap();
                *settings_guard = loaded_settings;
            }
        });
    }

    pub fn load_certificates(&self) {
        let certificate_repository = self.certificate_repository.clone();
        let certificates = Arc::clone(&self.certificates);

        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(Some(loaded_certificates)) = certificate_repository
                .get_certificates(CERTIFICATE_STORAGE_KEY)
                .await
            {
                let mut certificates_guard = certificates.write().unwrap();
                *certificates_guard = loaded_certificates;
            }
        });
    }
}
