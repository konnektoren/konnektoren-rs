use crate::components::{challenge::ChallengeComponent, game_path::GamePathComponent};
use konnektoren_core::prelude::*;
use log;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    let game = Game::default();
    let challenge: UseStateHandle<Option<Challenge>> = use_state(|| None);

    let game_copy = game.clone();
    let challenge_copy = challenge.clone();

    let new_challenge_cb = Callback::from(move |challenge_config: ChallengeConfig| {
        let challenge = &challenge_copy;
        match &game_copy.create_challenge(&challenge_config.id) {
            Ok(c) => challenge.set(Some(c.clone())),
            Err(err) => log::error!("Error creating challenge: {:?}", err),
        }
    });

    html! {
        <div class="app">
            <GamePathComponent game_path={game.game_path.clone()} on_challenge_config={new_challenge_cb} />
            {render_challenge(&*challenge)}
        </div>
    }
}

fn render_challenge(challenge: &Option<Challenge>) -> Html {
    if let Some(challenge) = &challenge {
        html! {
            <ChallengeComponent challenge={challenge.clone()} />
        }
    } else {
        html! {}
    }
}
