use crate::components::game_path::GamePathComponent;
use konnektoren_core::prelude::*;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    let game = Game {
        game_path: GamePath::default(),
        challenge_factory: ChallengeFactory::default(),
    };

    html! {
        <div class="app">
            <GamePathComponent game_path={game.game_path.clone()} />
        </div>
    }
}
