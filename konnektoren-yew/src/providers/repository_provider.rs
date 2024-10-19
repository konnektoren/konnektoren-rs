use crate::repository::{CertificateRepository, SettingsRepository, Storage};
use std::sync::Arc;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct RepositoryContext<S: Storage + Send + Sync + 'static> {
    pub certificate_repository: Arc<CertificateRepository<S>>,
    pub settings_repository: Arc<SettingsRepository<S>>,
}

impl<S: Storage + Send + Sync + 'static> RepositoryContext<S> {
    pub fn new(storage: S) -> Self {
        Self {
            certificate_repository: Arc::new(CertificateRepository::new(storage.clone())),
            settings_repository: Arc::new(SettingsRepository::new(storage.clone())),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct RepositoryProviderProps<S: Storage + Send + Sync + 'static> {
    pub children: Children,
    pub storage: S,
}

#[function_component(RepositoryProvider)]
pub fn repository_provider<S: Storage + Send + Sync + 'static>(
    props: &RepositoryProviderProps<S>,
) -> Html {
    let context = RepositoryContext::new(props.storage.clone());

    html! {
        <ContextProvider<RepositoryContext<S>> context={context}>
            { for props.children.iter() }
        </ContextProvider<RepositoryContext<S>>>
    }
}

#[hook]
pub fn use_certificate_repository<S: Storage + Send + Sync + 'static>(
) -> Arc<CertificateRepository<S>> {
    let context = use_context::<RepositoryContext<S>>().expect("RepositoryContext not found");
    context.certificate_repository.clone()
}

#[hook]
pub fn use_settings_repository<S: Storage + Send + Sync + 'static>() -> Arc<SettingsRepository<S>> {
    let context = use_context::<RepositoryContext<S>>().expect("RepositoryContext not found");
    context.settings_repository.clone()
}
