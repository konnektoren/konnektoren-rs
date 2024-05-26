use yew::prelude::*;

pub enum ChallengeActions {
    Next,
    Previous,
    Help,
}

#[derive(Properties, PartialEq)]
pub struct ChallengeActionsComponentProps {
    pub on_action: Callback<ChallengeActions>,
}

#[function_component(ChallengeActionsComponent)]
pub fn challenge_actions_component(props: &ChallengeActionsComponentProps) -> Html {
    html! {
        <div class="challenge-actions">
            <button onclick={props.on_action.reform(|_| ChallengeActions::Previous)}>{"Previous"}</button>
            <button onclick={props.on_action.reform(|_| ChallengeActions::Next)}>{"Next"}</button>
            <button onclick={props.on_action.reform(|_| ChallengeActions::Help)}>{"Help"}</button>
        </div>
    }
}
