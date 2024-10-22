use serde::{Deserialize, Serialize};

use crate::{game::GameState, player_profile::PlayerProfile};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Session {
    pub id: String,
    pub player_profile: PlayerProfile,
    pub game_state: GameState,
}

impl Session {
    pub fn new(id: String) -> Self {
        let player_profile = PlayerProfile::new(id.clone());
        Session {
            id,
            player_profile,
            game_state: GameState::default(),
        }
    }

    pub fn new_with_profile(player_profile: PlayerProfile) -> Self {
        Session {
            id: player_profile.id.clone(),
            player_profile,
            game_state: GameState::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_session() {
        let id = "123".to_string();
        let session = Session::new(id.clone());
        assert_eq!(session.id, id);
        assert_eq!(session.player_profile.id, id);
        assert_eq!(
            session.game_state.game.game_paths[0].challenge_ids().len(),
            5
        );
    }
}
