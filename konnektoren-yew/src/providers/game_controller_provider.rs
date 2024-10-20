use konnektoren_core::commands::CommandBus;
use konnektoren_core::controller::GameController;
use konnektoren_core::events::EventBus;
use konnektoren_core::game::{Game, GameState};
use konnektoren_core::persistence::MemoryPersistence;
use std::sync::{Arc, Mutex};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct GameControllerContext {
    pub controller: Arc<GameController>,
}

impl GameControllerContext {
    pub fn new(controller: Arc<GameController>) -> Self {
        Self { controller }
    }
}

#[hook]
pub fn use_game_controller() -> GameControllerContext {
    use_context::<GameControllerContext>().expect("GameControllerContext not found")
}

#[hook]
pub fn use_game_state() -> Arc<Mutex<GameState>> {
    let ctx = use_game_controller();
    Arc::clone(ctx.controller.game_state())
}

#[hook]
pub fn use_event_bus() -> Arc<EventBus> {
    let ctx = use_game_controller();
    Arc::new(ctx.controller.event_bus().clone())
}

#[hook]
pub fn use_command_bus() -> Arc<CommandBus> {
    let ctx = use_game_controller();
    Arc::new(ctx.controller.command_bus().clone())
}

#[derive(Properties, PartialEq)]
pub struct GameControllerProviderProps {
    pub children: Children,
}

#[function_component(GameControllerProvider)]
pub fn game_controller_provider(props: &GameControllerProviderProps) -> Html {
    let game = Game::default();
    let persistence = Arc::new(MemoryPersistence::default());
    let controller = GameController::new(game, persistence).init();
    let context = GameControllerContext::new(controller);

    html! {
        <ContextProvider<GameControllerContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<GameControllerContext>>
    }
}