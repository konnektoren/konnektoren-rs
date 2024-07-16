use konnektoren_core::challenges::ChallengeHistory;
use konnektoren_core::prelude::{Challenge, Performance};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub challenge_history: ChallengeHistory,
}

#[function_component(ChallengeHistorySummaryComponent)]
pub fn challenge_history_summary(props: &Props) -> Html {
    html! {
        <div class="challenge-history-summary">
            <h2>{ "Challenge History" }</h2>
            <div class="challenge-history">
                <h3>{ "Challenges" }</h3>
                { if props.challenge_history.challenges.is_empty() {
                    html! {
                        <p>{ "No challenges completed yet." }</p>
                    }
                } else {
                    html! {
                        <>
                        { format!("{}", props.challenge_history.challenges.len()) }
                        { " challenges completed." }
                        </>
                    }
                } }
                <table>
                    <thead>
                        <tr>
                            <th>{ "Challenge" }</th>
                            <th>{ "Result" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for props.challenge_history.challenges.iter().map(|history: &Challenge| {
                            html! {
                                <tr>
                                    <td>{ &history.challenge_type.name() }</td>
                                    <td>{ &history.performance(&history.challenge_result) }</td>
                                </tr>
                            }
                        }) }
                    </tbody>
                </table>
            </div>
        </div>
    }
}
