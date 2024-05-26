use super::{ChallengeActions, ChallengeActionsComponent, OptionsComponent, QuestionComponent};
use konnektoren_core::challenges::MultipleChoice;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MultipleChoiceComponentProps {
    pub challenge: MultipleChoice,
}

#[function_component(MultipleChoiceComponent)]
pub fn multiple_choice_component(props: &MultipleChoiceComponentProps) -> Html {
    let task_index = use_state(|| 0);
    let task_count = props.challenge.questions.len();

    let task_index_clone = task_index.clone();
    let on_action = Callback::from(move |action| {
        let task_index = task_index_clone.clone();
        match action {
            ChallengeActions::Next => {
                log::info!("Next");
                task_index.set((*task_index + 1) % task_count);
            }
            ChallengeActions::Previous => {
                log::info!("Previous");
                task_index.set((*task_index + task_count - 1) % task_count);
            }
            ChallengeActions::Help => {
                log::info!("Help");
            }
        }
    });

    html! {
        <div class="multiple-choice">
            <h2>{"Multiple Choice"}</h2>
            <p>{&props.challenge.id}</p>
            <p>{&props.challenge.name}</p>
            <QuestionComponent question={props.challenge.questions[*task_index].clone()} />
            <OptionsComponent options={props.challenge.options.clone()} />
            <ChallengeActionsComponent {on_action} />
        </div>
    }
}
