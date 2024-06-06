use konnektoren_core::challenges::Question;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct QuestionComponentProps {
    pub question: Question,
    #[prop_or_default]
    pub help: bool,
}

#[function_component(QuestionComponent)]
pub fn question_component(props: &QuestionComponentProps) -> Html {
    html! {
        <div class="question">
            <h2>{"Question"}</h2>
            <p>{&props.question.question}</p>
            <div class="help">
                {if props.help {
                    html! {
                        <p>{&props.question.help}</p>
                    }
                } else {
                    html! {}
                }}
            </div>
        </div>
    }
}
