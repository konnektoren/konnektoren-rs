use crate::i18n::config::I18nConfig;
use crate::i18n::SelectedLanguage;
use gloo::utils::window;
use yew::prelude::*;
use yew_i18n::{YewI18n, YewI18nConfig};

#[derive(Properties, Clone, PartialEq)]
pub struct I18nProviderProps {
    pub config: I18nConfig,
    pub children: Children,
}

#[function_component(I18nProvider)]
pub fn i18n_provider(props: &I18nProviderProps) -> Html {
    let browser_language = window()
        .navigator()
        .language()
        .unwrap_or_else(|| props.config.default_language.clone());
    let selected_language = if props
        .config
        .supported_languages
        .contains(&&browser_language.as_str())
    {
        SelectedLanguage::new(browser_language.as_str())
    } else {
        SelectedLanguage::new(props.config.default_language.as_str())
    };

    let mut i18n = YewI18n::new(
        YewI18nConfig {
            supported_languages: props.config.supported_languages.clone(),
            translations: props.config.translations.clone(),
        },
        props.config.translations.clone(),
    )
    .expect("Failed to initialize YewI18n");

    i18n.set_translation_language(&selected_language.get()).ok();

    let selected_language_ctx = use_state(|| selected_language);

    let i18n_ctx = use_state(|| i18n);

    html! {
        <ContextProvider<SelectedLanguage> context={(*selected_language_ctx).clone()}>{ html! {
            <ContextProvider<YewI18n> context={(*i18n_ctx).clone()}>{ props.children.clone() }</ContextProvider<YewI18n>>
        } }</ContextProvider<SelectedLanguage>>
    }
}

#[hook]
pub fn use_i18n() -> YewI18n {
    use_context::<YewI18n>().expect("No I18n context provided")
}

#[hook]
pub fn use_selected_language() -> SelectedLanguage {
    use_context::<SelectedLanguage>().expect("No SelectedLanguage context provided")
}
