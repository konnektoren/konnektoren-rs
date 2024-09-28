use crate::components::ChallengeEvent;
use crate::i18n::{I18nLoader, I18nYmlLoader, SelectedLanguage};
use konnektoren_core::challenges::{
    ChallengeResult, Custom, KonnektorenJs, Package, PackageReader,
};
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct CustomPackageComponentProps {
    pub challenge: Custom,
    #[prop_or_default]
    pub on_finish: Option<Callback<ChallengeResult>>,
    #[prop_or_default]
    pub on_event: Option<Callback<ChallengeEvent>>,
}

#[function_component(CustomPackageComponent)]
pub fn custom_package_component(props: &CustomPackageComponentProps) -> Html {
    let package = use_state(|| None::<Package>);
    let loading = use_state(|| true);
    let konnektoren_js = use_state(|| KonnektorenJs::new());

    // Effect to load the package
    {
        let package = package.clone();
        let challenge = props.challenge.clone();
        let loading = loading.clone();

        use_effect_with(challenge.clone(), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(package_url) = &challenge.package_url {
                    match PackageReader::download(package_url).await {
                        Ok(package_data) => match PackageReader::read(&package_data) {
                            Ok(loaded_package) => {
                                package.set(Some(loaded_package));
                                loading.set(false);
                            }
                            Err(e) => log::error!("Failed to read package: {}", e),
                        },
                        Err(e) => log::error!("Failed to download package: {}", e),
                    }
                }
            });
            || ()
        });
    }

    // Effect to set up the sendEvent callback once on mount
    {
        let konnektoren_js = konnektoren_js.clone();
        let on_event = props.on_event.clone();

        use_effect(move || {
            let on_event = on_event.clone();
            konnektoren_js.expose_send_event(move |event: JsValue| {
                if let Some(on_event_callback) = &on_event {
                    let challenge_event: ChallengeEvent = event.into();
                    on_event_callback.emit(challenge_event);
                }
            });
            || ()
        });
    }

    // Effect to process the loaded package after loading is complete
    {
        let konnektoren_js = konnektoren_js.clone();
        let package = package.clone();
        let loading = loading.clone();

        use_effect_with((loading, package.clone()), move |(loading, package)| {
            if !**loading {
                if let Some(loaded_package) = &**package {
                    // Set challenge data
                    if let Some(custom_challenge) = loaded_package.get_custom_challenge() {
                        konnektoren_js.set_challenge_data(custom_challenge);
                    }

                    // Set i18n data if available
                    if let Some(i18n_content) = loaded_package.get_file_as_string("i18n.yml") {
                        let language = SelectedLanguage::default().get();
                        let loader = I18nYmlLoader::new(i18n_content.as_str());
                        let translations = loader.get(&language).unwrap_or_default();
                        konnektoren_js.set_i18n_data(translations);
                    }

                    // Execute JS code
                    if let Some(js_content) = loaded_package.get_js_file() {
                        konnektoren_js.execute_js(js_content.as_str());
                    }
                }
            }
        });
    }

    // Render the HTML content
    html! {
        <div class="custom-package-challenge">
            if *loading {
                <p>{"Loading package..."}</p>
            } else if let Some(loaded_package) = &*package {
                if let Some(html_content) = loaded_package.get_html_file() {
                    <div>
                        if let Some(css_content) = loaded_package.get_css_file() {
                            <style>
                                {css_content}
                            </style>
                        }
                        {Html::from_html_unchecked(AttrValue::from(html_content))}
                    </div>
                } else {
                    <p>{"Error: HTML content not found in package"}</p>
                }
            } else {
                <p>{"Error: Failed to load package"}</p>
            }
        </div>
    }
}
