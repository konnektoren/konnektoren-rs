use super::{OptionsComponent, QuestionComponent};
use konnektoren_core::challenges::MultipleChoice;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MultipleChoiceComponentProps {
    pub challenge: MultipleChoice,
}

#[function_component(MultipleChoiceComponent)]
pub fn multiple_choice_component(props: &MultipleChoiceComponentProps) -> Html {
    html! {
        <div class="multiple-choice">
            <h2>{"Multiple Choice"}</h2>
            <p>{&props.challenge.id}</p>
            <p>{&props.challenge.name}</p>
            <QuestionComponent question={props.challenge.questions[0].clone()} />
            <OptionsComponent options={props.challenge.options.clone()} />
        </div>
    }
}
