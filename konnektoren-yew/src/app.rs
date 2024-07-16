use crate::components::{
    challenge::ChallengeComponent, game_map::GameMapComponent, game_path::GamePathComponent,
    MusicComponent,
};

#[cfg(feature = "storage")]
use crate::components::profile::{ProfileConfigComponent, ProfilePointsComponent};

use crate::components::challenge::SortTableComponent;
use crate::components::game_map::{ChallengeIndex, Coordinate};
use konnektoren_core::challenges::SortTable;
use konnektoren_core::prelude::*;
use log;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    let game = Game::default();
    let challenge: UseStateHandle<Option<Challenge>> = use_state(|| None);
    let sort_table_challenge = SortTable::default();

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
        Callback::from(
            move |(challenge_index, coords): (Option<ChallengeIndex>, Coordinate)| {
                let (x, y) = coords;
                if let Some(challenge_index) = challenge_index {
                    log::info!("Challenge index: {}, x: {}, y: {}", challenge_index, x, y);
                    if let Some(challenge_config) = game.game_path.challenges.get(challenge_index) {
                        match game.create_challenge(&challenge_config.id) {
                            Ok(c) => challenge.set(Some(c)),
                            Err(_) => log::error!("Challenge creation failed"),
                        }
                    }
                } else {
                    log::error!("Challenge not found");
                }
            },
        )
    };

    let profile_config_component = {
        #[cfg(feature = "storage")]
        html! {<ProfileConfigComponent />}
        #[cfg(not(feature = "storage"))]
        html! {<></>}
    };

    let profile_points_component = {
        #[cfg(feature = "storage")]
        html! {<ProfilePointsComponent />}
        #[cfg(not(feature = "storage"))]
        html! {<></>}
    };

    html! {
        <div class="app">
            <MusicComponent repeat={false} />
            {profile_config_component}
            {profile_points_component}
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
            <SortTableComponent challenge={sort_table_challenge} on_finish={Callback::noop()} on_event={Callback::noop()} />
        </div>
    }
}
