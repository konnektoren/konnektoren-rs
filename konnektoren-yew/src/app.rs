use crate::components::{
    challenge::ChallengeComponent, game_map::GameMapComponent, game_path::GamePathComponent,
};
use konnektoren_core::prelude::*;
use log;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    let game = Game::default();
    let challenge: UseStateHandle<Option<Challenge>> = use_state(|| None);

    let new_challenge_cb = {
        let game = game.clone();
        let challenge = challenge.clone();
        Callback::from(move |challenge_config: ChallengeConfig| {
            match game.create_challenge(&challenge_config.id) {
                Ok(c) => challenge.set(Some(c)),
                Err(err) => log::error!("Error creating challenge: {:?}", err),
            }
        })
    };

    let on_map_challenge_cb = {
        let game = game.clone();
        let challenge = challenge.clone();
        Callback::from(move |challenge_index: usize| {
            if let Some(challenge_config) = game.game_path.challenges.get(challenge_index) {
                match game.create_challenge(&challenge_config.id) {
                    Ok(c) => challenge.set(Some(c)),
                    Err(_) => log::error!("Challenge creation failed"),
                }
            } else {
                log::error!("Challenge not found");
            }
        })
    };

    html! {
        <div class="app">
            <GamePathComponent game_path={game.game_path.clone()} on_challenge_config={new_challenge_cb} />
            {
                if let Some(ref challenge) = *challenge {
                    html! { <ChallengeComponent challenge={challenge.clone()} /> }
                } else {
                    html! {}
                }
            }
            <GameMapComponent
                game_path={game.game_path.clone()}
                current_challenge={0}
                on_select_challenge={on_map_challenge_cb}
            />
        </div>
    }
}
