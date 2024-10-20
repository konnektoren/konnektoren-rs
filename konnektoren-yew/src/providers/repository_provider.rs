use crate::model::SessionInitializer;
use crate::repository::{
    CertificateRepository, CertificateRepositoryTrait, InboxRepository, InboxRepositoryTrait,
    ProfileRepository, ProfileRepositoryTrait, SessionRepository, SessionRepositoryTrait,
    SettingsRepository, SettingsRepositoryTrait, Storage,
};
use std::sync::Arc;
use yew::prelude::*;

#[derive(Clone)]
pub struct RepositoryConfig {
    pub certificate_repository: Arc<dyn CertificateRepositoryTrait>,
    pub settings_repository: Arc<dyn SettingsRepositoryTrait>,
    pub profile_repository: Arc<dyn ProfileRepositoryTrait>,
    pub inbox_repository: Arc<dyn InboxRepositoryTrait>,
    pub session_repository: Arc<dyn SessionRepositoryTrait>,
    pub session_initializer: Arc<dyn SessionInitializer>,
}

impl PartialEq for RepositoryConfig {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.certificate_repository, &other.certificate_repository)
            && Arc::ptr_eq(&self.settings_repository, &other.settings_repository)
            && Arc::ptr_eq(&self.profile_repository, &other.profile_repository)
            && Arc::ptr_eq(&self.inbox_repository, &other.inbox_repository)
            && Arc::ptr_eq(&self.session_repository, &other.session_repository)
            && Arc::ptr_eq(&self.session_initializer, &other.session_initializer)
    }
}

#[derive(Clone)]
pub struct RepositoryContext {
    pub certificate_repository: Arc<dyn CertificateRepositoryTrait>,
    pub settings_repository: Arc<dyn SettingsRepositoryTrait>,
    pub profile_repository: Arc<dyn ProfileRepositoryTrait>,
    pub inbox_repository: Arc<dyn InboxRepositoryTrait>,
    pub session_repository: Arc<dyn SessionRepositoryTrait>,
    pub session_initializer: Arc<dyn SessionInitializer>,
}

impl PartialEq for RepositoryContext {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.certificate_repository, &other.certificate_repository)
            && Arc::ptr_eq(&self.settings_repository, &other.settings_repository)
            && Arc::ptr_eq(&self.profile_repository, &other.profile_repository)
            && Arc::ptr_eq(&self.inbox_repository, &other.inbox_repository)
            && Arc::ptr_eq(&self.session_repository, &other.session_repository)
            && Arc::ptr_eq(&self.session_initializer, &other.session_initializer)
    }
}

impl RepositoryContext {
    pub fn new(config: RepositoryConfig) -> Self {
        Self {
            certificate_repository: config.certificate_repository,
            settings_repository: config.settings_repository,
            profile_repository: config.profile_repository,
            inbox_repository: config.inbox_repository,
            session_repository: config.session_repository,
            session_initializer: config.session_initializer,
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct RepositoryProviderProps {
    pub children: Children,
    pub config: RepositoryConfig,
}

#[function_component(RepositoryProvider)]
pub fn repository_provider(props: &RepositoryProviderProps) -> Html {
    let context = RepositoryContext::new(props.config.clone());

    html! {
        <ContextProvider<RepositoryContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<RepositoryContext>>
    }
}

pub fn create_repositories<S: Storage + Send + Sync + 'static>(
    storage: S,
    session_initializer: Arc<dyn SessionInitializer>,
) -> RepositoryConfig {
    RepositoryConfig {
        certificate_repository: Arc::new(CertificateRepository::new(storage.clone()))
            as Arc<dyn CertificateRepositoryTrait>,
        settings_repository: Arc::new(SettingsRepository::new(storage.clone()))
            as Arc<dyn SettingsRepositoryTrait>,
        profile_repository: Arc::new(ProfileRepository::new(storage.clone()))
            as Arc<dyn ProfileRepositoryTrait>,
        inbox_repository: Arc::new(InboxRepository::new(storage.clone()))
            as Arc<dyn InboxRepositoryTrait>,
        session_repository: Arc::new(SessionRepository::new(storage))
            as Arc<dyn SessionRepositoryTrait>,
        session_initializer,
    }
}
