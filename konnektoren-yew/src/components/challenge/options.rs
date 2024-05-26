use yew::prelude::*;

use konnektoren_core::challenges::MultipleChoiceOption;

#[derive(Properties, PartialEq)]
pub struct OptionsComponentProps {
    pub options: Vec<MultipleChoiceOption>,
}

#[function_component(OptionsComponent)]
pub fn options_component(props: &OptionsComponentProps) -> Html {
    html! {
        <div class="multiple-choice-options">
            {for props.options.iter().map(render_option)}
        </div>
    }
}

fn render_option(option: &MultipleChoiceOption) -> Html {
    html! {
        <div class="multiple-choice-option">
            <input type="radio" name="multiple-choice-option" id={option.id.to_string()} />
            <label for={option.id.to_string()}>{&option.name}</label>
        </div>
    }
}
