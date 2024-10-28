use crate::model::Inbox;
use crate::repository::{InboxRepositoryTrait, INBOX_STORAGE_KEY};
use std::sync::Arc;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct InboxContext {
    pub inbox: UseStateHandle<Inbox>,
}

#[derive(Properties)]
pub struct InboxProviderProps {
    pub children: Children,
    pub inbox_repository: Arc<dyn InboxRepositoryTrait>,
}

impl PartialEq for InboxProviderProps {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inbox_repository, &other.inbox_repository)
    }
}

#[function_component(InboxProvider)]
pub fn inbox_provider(props: &InboxProviderProps) -> Html {
    let inbox = use_state(|| Inbox::default());

    // Load inbox
    {
        let inbox = inbox.clone();
        let inbox_repository = props.inbox_repository.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(Some(loaded_inbox)) = inbox_repository.get_inbox(INBOX_STORAGE_KEY).await
                {
                    inbox.set(loaded_inbox);
                }
            });
            || ()
        });
    }

    {
        let inbox_repository = props.inbox_repository.clone();
        let current_inbox = (*inbox).clone();

        use_effect_with(current_inbox.clone(), move |_| {
            let inbox = current_inbox.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let inbox = inbox.clone();
                if let Err(e) = inbox_repository.save_inbox(INBOX_STORAGE_KEY, &inbox).await {
                    log::error!("Failed to save inbox: {:?}", e);
                }
            });
            || ()
        });
    }

    let context = InboxContext { inbox };

    html! {
        <ContextProvider<InboxContext> {context}>
            { for props.children.iter() }
        </ContextProvider<InboxContext>>
    }
}
