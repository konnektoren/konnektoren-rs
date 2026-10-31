use super::GameStatePersistence;
use crate::challenges::{ChallengeType, GameChallenge};
use crate::game::GameState;
use crate::persistence::error::{PersistenceError, Result};
use std::sync::Mutex;

#[derive(Debug)]
pub struct MemoryPersistence<C: GameChallenge = ChallengeType> {
    game_state: Mutex<GameState<C>>,
}

impl Default for MemoryPersistence {
    fn default() -> Self {
        Self::new(GameState::default())
    }
}

impl<C: GameChallenge> MemoryPersistence<C> {
    pub fn new(game_state: GameState<C>) -> Self {
        MemoryPersistence {
            game_state: Mutex::new(game_state),
        }
    }
}

impl<C: GameChallenge> GameStatePersistence<C> for MemoryPersistence<C> {
    fn save_game_state(&self, state: &GameState<C>) -> Result<()> {
        let mut stored = self.game_state.lock().map_err(|_| {
            PersistenceError::AccessError("Stored game state is poisoned".to_string())
        })?;
        *stored = state.clone();
        Ok(())
    }

    fn load_game_state(&self) -> Result<GameState<C>> {
        self.game_state
            .lock()
            .map(|stored| stored.clone())
            .map_err(|_| PersistenceError::AccessError("Stored game state is poisoned".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::GameState;

    #[test]
    fn test_save_game_state() {
        let persistence = MemoryPersistence::default();
        let state = GameState::default();

        let result = persistence.save_game_state(&state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_load_game_state() {
        let persistence = MemoryPersistence::default();
        let state = GameState::default();

        let _ = persistence.save_game_state(&state);

        let loaded_state = persistence.load_game_state().unwrap();
        assert_eq!(state, loaded_state);
    }

    #[test]
    fn test_poisoned_state_errors() {
        let persistence = MemoryPersistence::default();
        let state = GameState::default();

        // A panic while the state is locked poisons it, so later access fails
        let persistence = std::sync::Arc::new(persistence);
        let poisoner = persistence.clone();
        let _ = std::thread::spawn(move || {
            let _guard = poisoner.game_state.lock().unwrap();
            panic!("poison the stored game state");
        })
        .join();

        // This should fail with an AccessError
        let save_result = persistence.save_game_state(&state);
        assert!(save_result.is_err());

        if let Err(err) = save_result {
            match err {
                PersistenceError::AccessError(_) => {} // Expected error type
                _ => panic!("Unexpected error type: {:?}", err),
            }
        }

        // This should also fail with an AccessError
        let load_result = persistence.load_game_state();
        assert!(load_result.is_err());

        if let Err(err) = load_result {
            match err {
                PersistenceError::AccessError(_) => {} // Expected error type
                _ => panic!("Unexpected error type: {:?}", err),
            }
        }
    }

    #[test]
    fn concurrent_saves_and_loads_do_not_fail() {
        let persistence = std::sync::Arc::new(MemoryPersistence::default());
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let persistence = persistence.clone();
                std::thread::spawn(move || {
                    for _ in 0..50 {
                        persistence.save_game_state(&GameState::default()).unwrap();
                        persistence.load_game_state().unwrap();
                    }
                })
            })
            .collect();
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
