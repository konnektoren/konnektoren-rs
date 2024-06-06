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
    let challenge_result = use_state(|| ChallengeResult::default());
    let with_help = use_state(|| false);

    let task_count = props.challenge.questions.len();

    let task_index_clone = task_index.clone();
    let with_help_clone = with_help.clone();
    let on_action = Callback::from(move |action| {
        let task_index = task_index_clone.clone();
        let with_help = with_help_clone.clone();
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
                with_help.set(!*with_help);
            }
        }
    });

    let challenge_result_clone = challenge_result.clone();
    let on_option = Callback::from(move |option: MultipleChoiceOption| {
        log::info!("Option: {}", option.name);
        let mut challenge_result_update = (*challenge_result_clone).clone();
        challenge_result_update
            .add_input(ChallengeInput::MultipleChoice(option.clone()))
            .unwrap();
        challenge_result_clone.set(challenge_result_update);
    });

    html! {
        <div class="multiple-choice">
            <ProgressBar value={*task_index} max={task_count} label={format!("Question {} of {}", *task_index, task_count)}/>
            <QuestionComponent question={props.challenge.questions[*task_index].clone()} help={*with_help} />
            <OptionsComponent options={props.challenge.options.clone()} on_select={on_option} />
            <ChallengeActionsComponent {on_action} />
            <MultipleChoiceResultComponent challenge={props.challenge.clone()} challenge_result={(*challenge_result).clone()} />
        </div>
    }
}
