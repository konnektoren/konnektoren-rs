use std::rc::Rc;
use std::cell::RefCell;
use crate::components::ChallengeEvent;
use gloo::net::http::Request;
use gloo::utils::format::JsValueSerdeExt;
use konnektoren_core::challenges::{ChallengeResult, Custom};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
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

    // Store the closure inside Rc<RefCell<>> to persist it
    let js_event_callback = use_state(|| {
        Rc::new(RefCell::new(None::<Closure<dyn FnMut(JsValue)>>))
    });

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
        let event_callback = props.on_event.clone();

        let js_event_callback_clone = js_event_callback.clone();
        use_effect_with(event_callback.clone(), move |_| {
            let closure = Closure::wrap(Box::new(move |event: JsValue| {
                let event_callback = event_callback.clone();
                if let Some(event_callback) = event_callback.as_ref() {
                    let challenge_event: ChallengeEvent = event.into();
                    event_callback.emit(challenge_event);
                }
            }) as Box<dyn FnMut(JsValue)>);

            *js_event_callback_clone.borrow_mut() = Some(closure);

            || ()
        });

        let event_callback = props.on_event.clone();

        use_effect_with((js_code.clone(), event_callback.clone()), move |_| {
            let complete_js_code = format!("const challenge = {}; {}", challenge_json, js_code);

            let window = web_sys::window().unwrap();
            let global_obj = window.as_ref();
            let callback_name = JsValue::from_str("emit_challenge_event");

            if let Some(js_event_callback) = &*js_event_callback.borrow() {
                let callback_ref = js_event_callback.as_ref().unchecked_ref();
                if let Err(err) = js_sys::Reflect::set(global_obj, &callback_name, callback_ref) {
                    log::error!("Failed to set callback on window object: {:?}", err);
                }
            }

            if let Err(err) = eval(&complete_js_code) {
                log::error!("JavaScript execution failed: {:?}", err);
            }

            || () // Clean-up function
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
