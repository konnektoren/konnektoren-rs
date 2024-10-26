use super::repository_hooks::{use_session, use_session_repository};
use crate::repository::GameStatePersistenceImpl;
use konnektoren_core::commands::CommandBus;
use konnektoren_core::controller::{
    ChallengeFinishPlugin, ControllerPlugin, GameController, GameControllerTrait, GameXpPlugin,
};
use konnektoren_core::events::EventBus;
use konnektoren_core::game::{Game, GameState};
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
    #[prop_or_default]
    pub game_controller: Option<Arc<GameController>>,
}

#[function_component(GameControllerProvider)]
pub fn game_controller_provider(props: &GameControllerProviderProps) -> Html {
    let session_repository = use_session_repository();
    let session = use_session();

    let controller: Arc<GameController> = match &props.game_controller {
        Some(controller) => controller.clone(),
        None => {
            let game = Game::default();

            let persistence = Arc::new(GameStatePersistenceImpl {
                session_repository,
                session,
            });
            let mut controller = GameController::new(game, persistence);
            controller.register_plugin(Arc::new(ChallengeFinishPlugin));
            controller.register_plugin(Arc::new(GameXpPlugin));
            controller.init()
        }
    };

    controller.load_game_state().unwrap();
    let context = GameControllerContext::new(controller.clone());

    html! {
        <ContextProvider<GameControllerContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<GameControllerContext>>
    }
}
