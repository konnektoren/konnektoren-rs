use crate::components::ChallengeEvent;
use gloo::net::http::Request;
use konnektoren_core::challenges::{ChallengeResult, Custom, KonnektorenJs};
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct CustomComponentProps {
    pub challenge: Custom,
    #[prop_or_default]
    pub on_finish: Option<Callback<ChallengeResult>>,
    #[prop_or_default]
    pub on_event: Option<Callback<ChallengeEvent>>,
}

#[function_component(CustomComponent)]
pub fn custom_component(props: &CustomComponentProps) -> Html {
    let html_content = use_state(|| "".to_string());
    let css_content = use_state(|| "".to_string());
    let js_content = use_state(|| "".to_string());
    let loading = use_state(|| true);

    let konnektoren_js = use_state(|| KonnektorenJs::new());

    {
        let html_content = html_content.clone();
        let css_content = css_content.clone();
        let js_content = js_content.clone();
        let challenge = props.challenge.clone();
        let loading = loading.clone();

        use_effect_with(challenge.clone(), move |_| {
            wasm_bindgen_futures::spawn_local({
                let html_content = html_content.clone();
                let css_content = css_content.clone();
                let js_content = js_content.clone();
                let challenge = challenge.clone();
                async move {
                    fetch_content(&challenge.html, html_content).await;
                    fetch_content(&challenge.css, css_content).await;
                    fetch_content(&challenge.js, js_content).await;
                    loading.set(false);
                }
            });
            || ()
        });
    }

    {
        let challenge = props.challenge.clone();
        let js_code = (*js_content).clone();
        let on_event = props.on_event.clone();
        let konnektoren_js = konnektoren_js.clone();

        use_effect_with((challenge.clone(), js_code.clone()), move |_| {
            konnektoren_js.set_challenge_data(challenge);

            konnektoren_js.expose_send_event(move |event: JsValue| {
                if let Some(on_event_callback) = &on_event {
                    let challenge_event: ChallengeEvent = event.into();
                    on_event_callback.emit(challenge_event);
                }
            });
            konnektoren_js.execute_js(&js_code);

            || ()
        });
    }

    let parsed_html = Html::from_html_unchecked(AttrValue::from((*html_content).clone()));

    html! {
        <div class="custom-challenge">
            <style>
                {(*css_content).clone()}
            </style>
            {parsed_html}
        </div>
    }
}

pub async fn fetch_content(path: &str, handle: UseStateHandle<String>) {
    match fetch_file(path).await {
        Ok(content) => handle.set(content),
        Err(err) => log::error!("Failed to fetch the file content of {}: {}", path, err),
    }
}

pub async fn fetch_file(path: &str) -> Result<String, String> {
    let header_value = match path.split('.').last() {
        Some("js") => "application/javascript",
        Some("css") => "text/css",
        Some("html") => "text/html",
        _ => "text/plain",
    };

    let response = Request::get(path)
        .header("Accept", header_value)
        .send()
        .await
        .map_err(|_| format!("Failed to fetch the file {}", path))?;
    if response.status() == 200 {
        response
            .text()
            .await
            .map_err(|_| format!("Failed to read the file content of {}", path))
    } else {
        Err(format!("File not found: {}", path))
    }
}
