use crate::model::Inbox;
use crate::providers::use_inbox;
use crate::repository::INBOX_STORAGE_KEY;
use chrono::Utc;
use yew::prelude::*;
use yew_chat::prelude::Message;

#[function_component(InboxComponent)]
pub fn inbox_component() -> Html {
    let inbox = use_inbox();
    let is_open = use_state(|| false);
    let unread_count = use_state(|| 0);
    let inbox_repository = use_inbox_repository();

    // Calculate unread count
    {
        let inbox = inbox.clone();
        let unread_count = unread_count.clone();
        use_effect_with(inbox, move |inbox| {
            let unread = inbox.messages.len() - inbox.read_messages.as_ref().map_or(0, |v| v.len());
            unread_count.set(unread);
        });
    }

    let toggle_inbox = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
        })
    };

    let mark_as_read = {
        let inbox = inbox.clone();
        let unread_count = unread_count.clone();
        let inbox_repository = inbox_repository.clone();
        Callback::from(move |message_id: String| {
            let inbox = inbox.clone();
            let unread_count = unread_count.clone();
            let inbox_repository = inbox_repository.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(()) = inbox_repository
                    .mark_as_read(INBOX_STORAGE_KEY, &message_id)
                    .await
                {
                    let mut updated_inbox = (*inbox).clone();
                    let read_messages = updated_inbox.read_messages.get_or_insert_with(Vec::new);
                    if !read_messages.contains(&message_id) {
                        read_messages.push(message_id);
                        inbox.set(updated_inbox);
                        let current_unread = *unread_count;
                        unread_count.set(current_unread - 1);
                    }
                }
            });
        })
    };

    html! {
        <div class={classes!("inbox-component", if *is_open { "open" } else { "" })}>
            if *is_open {
                <div class="inbox-content">
                    <button class="close-button" onclick={toggle_inbox.clone()}>{"×"}</button>
                    <h2>{"Inbox"}</h2>
                    <div class="message-list">
                        {for inbox.messages.iter().map(|message| {
                            let is_read = inbox.read_messages.as_ref()
                                .map(|read| read.contains(&message.id.clone().unwrap_or_default()))
                                .unwrap_or(false);
                            let mark_as_read = mark_as_read.clone();
                            let message_id = message.id.clone().unwrap_or_default();
                            html! {
                                <div
                                    class={classes!("message", if is_read { "read" } else { "unread" })}
                                    onclick={Callback::from(move |_| mark_as_read.emit(message_id.clone()))}
                                >
                                    <div class="message-header">
                                        <span class="sender">{&message.sender}</span>
                                        <span class="timestamp">{message.timestamp.with_timezone(&Utc).format("%Y-%m-%d %H:%M").to_string()}</span>
                                    </div>
                                    <div class="message-content">{&message.content}</div>
                                </div>
                            }
                        })}
                    </div>
                </div>
            }
            <div class="inbox-icon" onclick={toggle_inbox}>
                <i class="fa-solid fa-envelope"></i>
                if *unread_count > 0 {
                    <span class="unread-count">{*unread_count}</span>
                }
            </div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(InboxComponent, (),);
}
