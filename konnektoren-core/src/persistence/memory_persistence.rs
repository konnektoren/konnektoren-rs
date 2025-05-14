use super::GameStatePersistence;
use crate::game::GameState;
use crate::persistence::error::{PersistenceError, Result};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct MemoryPersistence {
    game_state: Rc<RefCell<GameState>>,
}

unsafe impl Send for MemoryPersistence {}
unsafe impl Sync for MemoryPersistence {}

impl MemoryPersistence {
    pub fn new(game_state: GameState) -> Self {
        MemoryPersistence {
            game_state: Rc::new(RefCell::new(game_state)),
        }
    }
}

impl GameStatePersistence for MemoryPersistence {
    fn save_game_state(&self, state: &GameState) -> Result<()> {
        match self.game_state.try_borrow_mut() {
            Ok(mut gs) => {
                *gs = state.clone();
                Ok(())
            }
            Err(_) => Err(PersistenceError::AccessError(
                "Failed to get mutable borrow of game state".to_string(),
            )),
        }
    }

    fn load_game_state(&self) -> Result<GameState> {
        self.game_state
            .try_borrow()
            .map(|gs| gs.clone())
            .map_err(|_| PersistenceError::AccessError("Failed to borrow game state".to_string()))
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
    fn test_concurrent_access_errors() {
        let persistence = MemoryPersistence::default();
        let state = GameState::default();

        // Create a borrow that will cause subsequent borrows to fail
        let _borrow = persistence.game_state.borrow_mut();

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
}
