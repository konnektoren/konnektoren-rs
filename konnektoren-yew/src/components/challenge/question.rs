use konnektoren_core::challenges::Question;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct QuestionComponentProps {
    pub question: Question,
}

#[function_component(QuestionComponent)]
pub fn question_component(props: &QuestionComponentProps) -> Html {
    html! {
        <div class="question">
            <h2>{"Question"}</h2>
            <p>{&props.question.question}</p>
            <p>{&props.question.help}</p>
        </div>
    }
}
