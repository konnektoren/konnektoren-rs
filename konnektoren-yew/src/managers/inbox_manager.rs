use crate::components::InboxComponent;
use crate::model::Inbox;
use crate::providers::use_inbox_repository;
use crate::repository::INBOX_STORAGE_KEY;
use gloo::net::http::Request;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InboxManagerProps {
    #[prop_or_default]
    pub children: Option<ChildrenWithProps<InboxComponent>>,
}

#[function_component(InboxManager)]
pub fn inbox_manager(props: &InboxManagerProps) -> Html {
    let inbox_state = use_state(Inbox::default);
    let inbox_repo = use_inbox_repository();

    {
        let inbox_state = inbox_state.clone();
        let inbox_repo = inbox_repo.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let yaml_content = Request::get("/assets/inbox.yml")
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                let fetched_inbox: Inbox = serde_yaml::from_str(&yaml_content).unwrap();

                let stored_inbox = inbox_repo
                    .get_inbox(INBOX_STORAGE_KEY)
                    .await
                    .unwrap_or_default()
                    .unwrap_or_default();
                let mut merged_inbox = stored_inbox.clone();
                merged_inbox.merge(&fetched_inbox);

                merged_inbox
                    .messages
                    .sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

                inbox_state.set(merged_inbox.clone());

                inbox_repo
                    .save_inbox(INBOX_STORAGE_KEY, &merged_inbox)
                    .await
                    .unwrap();
            });
        });
    }

    let mark_as_read = {
        let inbox_state = inbox_state.clone();
        let inbox_repo = inbox_repo.clone();
        Callback::from(move |message_id: String| {
            let inbox_state = inbox_state.clone();
            let inbox_repo = inbox_repo.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let mut current_inbox = (*inbox_state).clone();
                let read_messages = current_inbox.read_messages.get_or_insert_with(Vec::new);
                if !read_messages.contains(&message_id) {
                    read_messages.push(message_id);
                    inbox_state.set(current_inbox.clone());
                    inbox_repo
                        .save_inbox(INBOX_STORAGE_KEY, &current_inbox)
                        .await
                        .unwrap();
                }
            });
        })
    };

    match &props.children {
        Some(children) => {
            let modified_children = children.iter().map(|mut item| {
                let props = Rc::make_mut(&mut item.props);
                props.inbox = (&*inbox_state).clone();
                props.on_read_message = mark_as_read.clone();
                item
            });
            html! { for modified_children }
        }
        None => {
            return html! {
                <InboxComponent inbox={(&*inbox_state).clone()} on_read_message={mark_as_read.clone()} />
            }
        }
    }
}
