use cucumber::World;
pub mod steps;
use cucumber::WriterExt;
use konnektoren_core::prelude::*;
use std::boxed::Box;

#[derive(Debug, World)]
pub struct BddWorld {
    pub session: Session,
    pub challenge_type: ChallengeType,
    pub challenge: Option<Challenge>,
    pub challenge_result: Option<ChallengeResult>,
    pub game_path: GamePath,
    pub factory: Option<ChallengeFactory>,
    pub game: Game,
}

impl Default for BddWorld {
    fn default() -> Self {
        let session = Session::new("123".to_string());
        let challenge_type = ChallengeType::default();
        let game_path = GamePath::default();
        let mut factory = ChallengeFactory::new();
        factory.challenge_types.push(challenge_type.clone());
        let game = Game {
            game_path: game_path.clone(),
            challenge_factory: factory.clone(),
            challenge_history: Default::default(),
        };

        let world = Self {
            session,
            challenge: None,
            challenge_type,
            challenge_result: None,
            game_path,
            factory: Some(factory),
            game,
        };
        world
    }
}

#[tokio::main]
async fn main() {
    BddWorld::cucumber()
        .max_concurrent_scenarios(1)
        .with_writer(
            cucumber::writer::Basic::raw(std::io::stdout(), cucumber::writer::Coloring::Never, 0)
                .summarized()
                .assert_normalized(),
        )
        .run_and_exit("tests/features")
        .await;
}
