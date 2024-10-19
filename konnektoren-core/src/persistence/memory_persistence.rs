use super::GameStatePersistence;
use crate::game::GameState;
use anyhow::Result;
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
        *self.game_state.borrow_mut() = state.clone();
        Ok(())
    }

    fn load_game_state(&self) -> Result<GameState> {
        Ok(self.game_state.borrow().clone())
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
}
