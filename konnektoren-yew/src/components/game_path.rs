use konnektoren_core::prelude::*;
use yew::prelude::*;

use crate::components::challenge_config::ChallengeConfigComponent;

#[derive(Properties, PartialEq)]
pub struct GamePathComponentProps {
    pub game_path: GamePath,
}

#[function_component(GamePathComponent)]
pub fn game_path_component(props: &GamePathComponentProps) -> Html {
    html! {
        <div class="game-path">
            <h1>{&props.game_path.name}</h1>
            <ul>
                {for props.game_path.challenges.iter().map(|challenge| html! {
                    <li>
                    <ChallengeConfigComponent challenge_config={challenge.clone()} />
                    </li>
                })}
            </ul>
        </div>
    }
}
