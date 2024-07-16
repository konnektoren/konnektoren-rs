use konnektoren_core::challenges::challenge_config::ChallengeVariant;
use konnektoren_core::prelude::*;
use yew::prelude::*;

mod actions;
mod events;
mod multiple_choice;
mod multiple_choice_circle;
mod multiple_choice_result;
mod options;
mod question;
mod result_summary;
mod sort_table;
mod sort_table_result;

use crate::components::ChallengeInfoComponent;
pub use actions::{ChallengeActions, ChallengeActionsComponent};
pub use events::ChallengeEvent;
pub use multiple_choice::MultipleChoiceComponent;
pub use multiple_choice_circle::MultipleChoiceCircleComponent;
pub use multiple_choice_result::MultipleChoiceResultComponent;
pub use options::OptionsComponent;
pub use question::QuestionComponent;
pub use result_summary::ResultSummaryComponent;
pub use sort_table::SortTableComponent;
pub use sort_table_result::SortTableResultComponent;

#[derive(Properties, PartialEq)]
pub struct ChallengeComponentProps {
    pub challenge: Challenge,
    #[prop_or_default]
    pub variant: Option<ChallengeVariant>,
    #[prop_or_default]
    pub on_finish: Option<Callback<ChallengeResult>>,
    #[prop_or_default]
    pub on_event: Option<Callback<ChallengeEvent>>,
}

#[function_component(ChallengeComponent)]
pub fn challenge_component(props: &ChallengeComponentProps) -> Html {
    let challenge_result = use_state(|| Option::<ChallengeResult>::None);
    let show_challenge_info = use_state(|| false);

    let handle_finish = {
        let challenge_result = challenge_result.clone();
        let on_finish = props.on_finish.clone();
        let on_event = props.on_event.clone();
        Callback::from(move |result: ChallengeResult| {
            log::info!("Challenge Result: {:?}", result);
            challenge_result.set(Some(result.clone()));
            if let Some(on_finish) = on_finish.as_ref() {
                on_finish.emit(result.clone());
            }
            if let Some(on_event) = on_event.as_ref() {
                on_event.emit(ChallengeEvent::Finish(result));
            }
        })
    };

    let handle_event = {
        let on_event = props.on_event.clone();
        Callback::from(move |event: ChallengeEvent| {
            if let Some(on_event) = on_event.as_ref() {
                on_event.emit(event);
            }
        })
    };

    let challenge_component = match (
        &*challenge_result,
        &props.challenge.challenge_type,
        &props.variant.clone().unwrap_or_default(),
    ) {
        (None, ChallengeType::MultipleChoice(challenge), ChallengeVariant::MultipleChoice) => {
            html! {
                <MultipleChoiceComponent challenge={challenge.clone()} on_finish={handle_finish} on_event={handle_event} />
            }
        }
        (
            None,
            ChallengeType::MultipleChoice(challenge),
            ChallengeVariant::MultipleChoiceCircle,
        ) => html! {
            <MultipleChoiceCircleComponent challenge={challenge.clone()} on_finish={handle_finish} on_event={handle_event} />
        },
        (None, ChallengeType::SortTable(challenge), ChallengeVariant::SortTable) => html! {
            <SortTableComponent challenge={challenge.clone()} on_finish={handle_finish} on_event={handle_event} />
        },
        _ => html! {},
    };

    let challenge_result_component = match (&*challenge_result, &props.challenge.challenge_type) {
        (Some(result), ChallengeType::MultipleChoice(challenge)) => html! {
            <MultipleChoiceResultComponent challenge={challenge.clone()} challenge_result={result.clone()} />
        },
        _ => html! {},
    };

    let challenge_info = {
        let show_info = *show_challenge_info;
        html! {
            <>
                <button class="challenge-info-button" onclick={
                Callback::from(move |_| show_challenge_info.set(!show_info))}>
                    {if show_info { "X" } else { "?" }}
                </button>
                <div class="challenge-info" style={if show_info { "display: block;" } else { "display: none;" }}>
                <ChallengeInfoComponent challenge_config={props.challenge.challenge_config.clone()} />
                </div>
            </>
        }
    };

    html! {
        <div class="challenge">
            {challenge_info}
            {challenge_component}
            {challenge_result_component}
        </div>
    }
}
