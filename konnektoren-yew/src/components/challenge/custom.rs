use crate::components::ChallengeEvent;
use gloo::net::http::Request;
use konnektoren_core::challenges::{ChallengeResult, Custom};
use wasm_bindgen_futures::js_sys::eval;
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
        let js_code = (*js_content).clone();
        let challenge_json = serde_json::to_string_pretty(&props.challenge).unwrap();
        use_effect_with(js_code.clone(), move |_| {
            let complete_js_code = format!("const challenge = {}; {}", challenge_json, js_code);
            if let Err(err) = eval(&complete_js_code) {
                log::error!("JavaScript execution failed: {:?}", err);
            }
            || ()
        });
    }

    let parsed = Html::from_html_unchecked(AttrValue::from((*html_content).clone()));

    html! {
        <div class="custom-challenge">
            <style>
                {(*css_content).clone()}
            </style>
            {parsed}
        </div>
    }
}

async fn fetch_content(path: &str, handle: UseStateHandle<String>) {
    match fetch_file(path).await {
        Ok(content) => handle.set(content),
        Err(err) => log::error!("Failed to fetch the file content of {}: {}", path, err),
    }
}

async fn fetch_file(path: &str) -> Result<String, String> {
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
