use super::{ChallengeActions, ChallengeActionsComponent, OptionsComponent, QuestionComponent};
use crate::components::challenge::multiple_choice_result::MultipleChoiceResultComponent;
use crate::components::ProgressBar;
use konnektoren_core::challenges::{MultipleChoice, MultipleChoiceOption};
use konnektoren_core::prelude::{ChallengeInput, ChallengeResult};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MultipleChoiceComponentProps {
    pub challenge: MultipleChoice,
}

#[function_component(MultipleChoiceComponent)]
pub fn multiple_choice_component(props: &MultipleChoiceComponentProps) -> Html {
    let task_index = use_state(|| 0);
    let challenge_result = use_state(ChallengeResult::default);
    let show_help = use_state(|| false);

    let handle_action = {
        let task_index = task_index.clone();
        let show_help = show_help.clone();
        let total_tasks = props.challenge.questions.len();

        Callback::from(move |action: ChallengeActions| match action {
            ChallengeActions::Next => {
                if *task_index < total_tasks - 1 {
                    task_index.set(*task_index + 1);
                }
            }
            ChallengeActions::Previous => {
                if *task_index > 0 {
                    task_index.set(*task_index - 1);
                }
            }
            ChallengeActions::Help => {
                show_help.set(!*show_help);
            }
        })
    };

    let handle_option_selection = {
        let task_index = task_index.clone();
        let challenge_result_clone = challenge_result.clone();
        let total_tasks = props.challenge.questions.len();

        Callback::from(move |option: MultipleChoiceOption| {
            let mut challenge_result_update = (*challenge_result_clone).clone();
            challenge_result_update
                .add_input(ChallengeInput::MultipleChoice(option.clone()))
                .unwrap();
            challenge_result_clone.set(challenge_result_update);

            if *task_index < total_tasks - 1 {
                task_index.set(*task_index + 1);
            }
        })
    };

    html! {
        <div class="multiple-choice">
            <ProgressBar
                value={*task_index}
                max={props.challenge.questions.len()}
                label={format!("Question {} of {}", *task_index + 1, props.challenge.questions.len())}
            />
            <QuestionComponent
                question={props.challenge.questions[*task_index].clone()}
                help={*show_help}
            />
            <OptionsComponent
                options={props.challenge.options.clone()}
                on_select={handle_option_selection}
            />
            <ChallengeActionsComponent on_action={handle_action} />
            <MultipleChoiceResultComponent
                challenge={props.challenge.clone()}
                challenge_result={(*challenge_result).clone()}
            />
        </div>
    }
}
