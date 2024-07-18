use konnektoren_core::challenges::{ChallengeResult, Informative};
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct InformativeComponentProps {
    pub challenge: Informative,
    #[prop_or_default]
    pub on_finish: Option<Callback<ChallengeResult>>,
    #[prop_or_default]
    pub language: Option<String>,
}

#[function_component(InformativeComponent)]
pub fn informative_component(props: &InformativeComponentProps) -> Html {
    let language = props.language.as_deref().unwrap_or("en");

    let on_finish = props.on_finish.clone();
    let on_finish = Callback::from(move |_| {
        if let Some(on_finish) = on_finish.as_ref() {
            on_finish.emit(ChallengeResult::default());
        }
    });

    let informative_text = props.challenge.text.iter().find(|t| t.language == language);

    let text = match informative_text {
        Some(text) => &text.text,
        None => "No text found",
    };

    html! {
        <div>
            <h1>{&props.challenge.description}</h1>
            <p>{text}</p>
            <button onclick={on_finish}>{"Finish"}</button>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::challenges::InformativeText;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        InformativeComponent,
        InformativeComponentProps {
            challenge: Informative {
                id: "".to_string(),
                name: "".to_string(),
                description: "Informative Challenge".to_string(),
                text: vec![InformativeText {
                    language: "en".to_string(),
                    text: "This is an informative challenge".to_string(),
                }],
            },
            on_finish: None,
            language: None,
        },
        (
            "unknown language",
            InformativeComponentProps {
                challenge: Informative {
                    id: "".to_string(),
                    name: "".to_string(),
                    description: "Informative Challenge".to_string(),
                    text: vec![InformativeText {
                        language: "en".to_string(),
                        text: "This is an informative challenge".to_string(),
                    }],
                },
                on_finish: None,
                language: Some("de".to_string()),
            }
        )
    );
}
