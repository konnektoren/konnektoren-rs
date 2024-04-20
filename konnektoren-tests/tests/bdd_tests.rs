use cucumber::World;
pub mod steps;
use cucumber::WriterExt;
use konnektoren_core::prelude::*;
use std::boxed::Box;

#[derive(Debug, World)]
pub struct BddWorld {
    session: Session,
    challenge: ChallengeType,
    game_path: GamePath,
}

impl Default for BddWorld {
    fn default() -> Self {
        let world = Self {
            session: Session::new("123".to_string()),
            challenge: ChallengeType::default(),
            game_path: GamePath::default(),
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
