use crate::i18n::{flag, use_i18n, use_selected_language, LANGUAGES};
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[function_component(SelectLanguage)]
pub fn select_language() -> Html {
    let i18n = use_i18n();
    let selected_language = use_selected_language();

    let on_select_change = {
        let selected_language = selected_language.clone();
        Callback::from(move |e: Event| {
            let mut selected_language = selected_language.clone();
            let select = e.target_dyn_into::<HtmlSelectElement>();
            if let Some(select) = select {
                let value = select.value();
                selected_language.set(&value.clone());
                web_sys::window().unwrap().location().reload().unwrap();
            }
        })
    };

    html! {
        <div class="select-language">
            <p>
                { i18n.t("Please select a language from the dropdown.") }
                <select onchange={on_select_change} value={(selected_language.get()).clone()}>
                    <option value="" selected={selected_language.get().is_empty()} disabled=true>{ i18n.t("Select Language") }</option>
                    { for LANGUAGES.iter().map(|&lang| html! {
                        <option value={lang} selected={*lang == *selected_language.get()}>{format!("{} {}", flag(lang), lang)}</option>
                    })}
                </select>
            </p>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(SelectLanguage, (),);
}
