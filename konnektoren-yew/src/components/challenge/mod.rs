use konnektoren_core::challenges::challenge_config::ChallengeVariant;
use konnektoren_core::prelude::*;
use yew::prelude::*;

mod actions;
mod challenge_history_summary;
pub mod custom;
mod custom_result;
mod events;
mod informative;
mod informative_markdown;
mod informative_result;
pub mod multiple_choice;
pub mod multiple_choice_circle;
mod multiple_choice_result;
mod options;
pub mod question;
pub mod result_summary;
pub mod sort_table;
pub mod sort_table_result;

use crate::components::ChallengeInfoComponent;
pub use actions::{ChallengeActions, ChallengeActionsComponent};
pub use challenge_history_summary::ChallengeHistorySummaryComponent;
pub use custom::CustomComponent;
pub use custom_result::CustomResultComponent;
pub use events::ChallengeEvent;
pub use informative::InformativeComponent;
pub use informative_markdown::InformativeMarkdownComponent;
pub use informative_result::InformativeResultComponent;
pub use multiple_choice::MultipleChoiceComponent;
pub use multiple_choice_circle::MultipleChoiceCircleComponent;
pub use multiple_choice_result::MultipleChoiceResultComponent;
pub use options::OptionsComponent;
pub use question::QuestionComponent;
pub use result_summary::ResultSummaryComponent;
pub use sort_table::SortTableComponent;
pub use sort_table_result::SortTableResultComponent;

#[derive(Properties, PartialEq, Default)]
pub struct ChallengeComponentProps {
    pub challenge: Challenge,
    #[prop_or_default]
    pub variant: Option<ChallengeVariant>,
    #[prop_or_default]
    pub on_finish: Option<Callback<ChallengeResult>>,
    #[prop_or_default]
    pub on_event: Option<Callback<ChallengeEvent>>,
    #[prop_or_default]
    pub language: Option<String>,
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
        (None, ChallengeType::Informative(challenge), ChallengeVariant::InformativeText) => html! {
            <InformativeComponent challenge={challenge.clone()} on_finish={handle_finish} language={props.language.clone()} />
        },
        (None, ChallengeType::Informative(challenge), ChallengeVariant::InformativeMarkdown) => {
            html! {
                <InformativeMarkdownComponent challenge={challenge.clone()} on_finish={handle_finish} language={props.language.clone()}  />
            }
        }
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

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(ChallengeComponent, ChallengeComponentProps::default(),);
}
