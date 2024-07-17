use crate::components::{
    challenge::ChallengeComponent, game_map::GameMapComponent, game_path::GamePathComponent,
    MusicComponent,
};

#[cfg(feature = "storage")]
use crate::components::profile::{ProfileConfigComponent, ProfilePointsComponent};

use crate::components::challenge::multiple_choice::MultipleChoiceComponentProps;
use crate::components::challenge::sort_table::SortTableComponentProps;
use crate::components::challenge::{
    ChallengeComponentProps, MultipleChoiceCircleComponent, MultipleChoiceComponent,
    SortTableComponent,
};
use crate::components::game_map::{ChallengeIndex, Coordinate, GameMapComponentProps};
use crate::components::game_path::GamePathComponentProps;
use crate::components::music::MusicComponentProps;
use konnektoren_core::prelude::*;
use log;
use yew::prelude::*;
use yew_preview::prelude::PreviewPage;
use yew_preview::{create_component_item, ComponentItem, ComponentList};

#[function_component]
pub fn Example() -> Html {
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
        <div>
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
        </div>
    }
}

#[function_component]
pub fn App() -> Html {
    let game = Game::default();
    let default_challenge = game.create_challenge("konnektoren-1").unwrap();
    let default_multiple_choice: MultipleChoice = match &default_challenge.challenge_type {
        ChallengeType::MultipleChoice(multiple_choice) => multiple_choice.clone(),
        _ => unreachable!(),
    };

    let component_list: ComponentList = vec![
        create_component_item!(
            "MusicComponent",
            MusicComponent,
            vec![
                (
                    "no repeat",
                    MusicComponentProps {
                        repeat: Some(false),
                        ..Default::default()
                    }
                ),
                ("default", MusicComponentProps::default())
            ]
        ),
        create_component_item!(
            "ProfileConfigComponent",
            ProfileConfigComponent,
            vec![("default", ())]
        ),
        create_component_item!(
            "GamePathComponent",
            GamePathComponent,
            vec![("default", GamePathComponentProps::default())]
        ),
        create_component_item!(
            "GameMapComponent",
            GameMapComponent,
            vec![("default", GameMapComponentProps::default())]
        ),
        create_component_item!(
            "MultipleChoiceComponent",
            MultipleChoiceComponent,
            vec![(
                "default",
                MultipleChoiceComponentProps {
                    challenge: default_multiple_choice.clone(),
                    ..Default::default()
                }
            )]
        ),
        create_component_item!(
            "MultipleChoiceCircleComponent",
            MultipleChoiceCircleComponent,
            vec![(
                "default",
                MultipleChoiceComponentProps {
                    challenge: default_multiple_choice.clone(),
                    ..Default::default()
                }
            )]
        ),
        create_component_item!(
            "SortTableComponent",
            SortTableComponent,
            vec![("default", SortTableComponentProps::default())]
        ),
        create_component_item!(
            "ChallengeComponent",
            ChallengeComponent,
            vec![("default", ChallengeComponentProps::default())]
        ),
        create_component_item!("Example", Example, vec![("default", ())]),
    ];

    html! {
        <div class="app">
            <PreviewPage components={component_list} />
        </div>
    }
}
