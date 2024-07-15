use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TranslateProps {
    pub text: String,
}

#[function_component(TranslateComponent)]
pub fn translate(props: &TranslateProps) -> Html {
    let google_translate_url = format!(
        "https://translate.google.com/?sl=auto&tl=en&text={}",
        props.text
    );
    html! {
        <div class="translate-button">
            <a href={google_translate_url} target="_blank" rel="noopener noreferrer">
                { "Translate" }
            </a>
        </div>
    }
}
