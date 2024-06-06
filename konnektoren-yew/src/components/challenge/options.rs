use yew::prelude::*;

use konnektoren_core::challenges::MultipleChoiceOption;

#[derive(Properties, PartialEq)]
pub struct OptionsComponentProps {
    pub options: Vec<MultipleChoiceOption>,
    pub on_select: Option<Callback<MultipleChoiceOption>>,
}

#[function_component(OptionsComponent)]
pub fn options_component(props: &OptionsComponentProps) -> Html {
    html! {
        <div class="multiple-choice-options">
            {for props.options.iter().map(|option| render_option(option, &props.on_select))}
        </div>
    }
}

fn render_option(
    option: &MultipleChoiceOption,
    on_select: &Option<Callback<MultipleChoiceOption>>,
) -> Html {
    let option_clone = option.clone();
    let on_select = on_select.clone();
    let on_select = Callback::from(move |_| {
        let option = option_clone.clone();
        if let Some(on_select) = on_select.as_ref() {
            on_select.emit(option);
        }
    });

    html! {
        <div class="multiple-choice-option">
            <input type="radio" name="multiple-choice-option" id={option.id.to_string()} onclick={on_select} />
            <label for={option.id.to_string()}>{&option.name}</label>
        </div>
    }
}
