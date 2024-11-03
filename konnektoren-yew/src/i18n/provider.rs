use crate::i18n::{I18nConfig, SelectedLanguage};
use crate::providers::use_settings;
use gloo::utils::window;
use yew::prelude::*;
use yew_i18n::{YewI18n, YewI18nConfig};

#[derive(Clone, PartialEq)]
pub struct I18nContext {
    pub i18n: UseStateHandle<YewI18n>,
    pub selected_language: SelectedLanguage,
}

#[derive(Properties, Clone, PartialEq)]
pub struct I18nProviderProps {
    pub config: I18nConfig,
    pub children: Children,
}

#[function_component(I18nProvider)]
pub fn i18n_provider(props: &I18nProviderProps) -> Html {
    let settings = use_settings();

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
        SelectedLanguage::new(&settings.language)
    };

    let mut i18n = YewI18n::new(
        YewI18nConfig {
            supported_languages: props.config.supported_languages.clone(),
            translations: props.config.translations.clone(),
        },
        props.config.translations.clone(),
    )
    .expect("Failed to initialize YewI18n");

    i18n.set_translation_language(&selected_language.language)
        .unwrap();

    let i18n_ctx = use_state(|| i18n);

    {
        let i18n_ctx = i18n_ctx.clone();
        let settings = settings.clone();

        use_effect_with(settings.clone(), move |settings| {
            let settings = settings.clone();
            let mut i18n = (&*i18n_ctx).clone();
            i18n.set_translation_language(&(*settings.language))
                .unwrap();
            i18n_ctx.set(i18n);
        });
    }

    let context = I18nContext {
        i18n: i18n_ctx,
        selected_language,
    };

    html! {
        <ContextProvider<I18nContext> {context}>
            { for props.children.iter() }
        </ContextProvider<I18nContext>>
    }
}

#[hook]
pub fn use_i18n() -> UseStateHandle<YewI18n> {
    use_context::<I18nContext>()
        .expect("No I18n context provided")
        .i18n
        .clone()
}

#[hook]
pub fn use_selected_language() -> SelectedLanguage {
    use_context::<I18nContext>()
        .expect("No I18n context provided")
        .selected_language
}
