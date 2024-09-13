use crate::components::custom::fetch_content;
use konnektoren_core::challenges::{Custom, CustomChallengeResult, KonnektorenJs};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct CustomResultComponentProps {
    pub challenge: Custom,
    pub result: CustomChallengeResult,
}

#[function_component(CustomResultComponent)]
pub fn custom_result(props: &CustomResultComponentProps) -> Html {
    if props.challenge.results_html.is_none() {
        return html! {
            <div class="custom-result">
            </div>
        };
    }

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
                    fetch_content(&challenge.results_html.as_ref().unwrap(), html_content).await;
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
        let result = props.result.clone();
        let js_code = (*js_content).clone();
        let konnektoren_js = konnektoren_js.clone();

        use_effect_with((challenge.clone(), js_code.clone()), move |_| {
            konnektoren_js.set_challenge_data(challenge);
            konnektoren_js.set_result_data(result.clone());
            konnektoren_js.execute_js(&js_code);

            || ()
        });
    }

    let parsed_html = Html::from_html_unchecked(AttrValue::from((*html_content).clone()));

    html! {
        <div class="custom-result">
            <style>
                {(*css_content).clone()}
            </style>
            {parsed_html}
        </div>
    }
}
