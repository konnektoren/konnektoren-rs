use serde::{Deserialize, Serialize};

use crate::{
    challenges::{ChallengeType, GameChallenge},
    game::GameState,
    player_profile::PlayerProfile,
};

/// A player's saved progress. Applications with their own challenge type use
/// `Session<TheirType>`; the saved form of built-in challenges is unchanged.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(bound(
    serialize = "C: Serialize, C::Result: Serialize, C::Variant: Serialize",
    deserialize = "C: Deserialize<'de>, C::Result: Deserialize<'de>, C::Variant: Deserialize<'de>"
))]
pub struct Session<C: GameChallenge = ChallengeType> {
    pub id: String,
    pub player_profile: PlayerProfile,
    pub game_state: GameState<C>,
}

impl Default for Session {
    fn default() -> Self {
        Session {
            id: String::new(),
            player_profile: PlayerProfile::default(),
            game_state: GameState::default(),
        }
    }
}

impl<C: GameChallenge> Session<C> {
    /// A session for `id` that starts from the given game state.
    pub fn new_with_state(id: String, game_state: GameState<C>) -> Self {
        let player_profile = PlayerProfile::new(id.clone());
        Session {
            id,
            player_profile,
            game_state,
        }
    }

    /// Like [`Session::new_with_state`], for an existing player profile.
    pub fn new_with_profile_and_state(
        player_profile: PlayerProfile,
        game_state: GameState<C>,
    ) -> Self {
        Session {
            id: player_profile.id.clone(),
            player_profile,
            game_state,
        }
    }
}

impl Session {
    pub fn new(id: String) -> Self {
        Self::new_with_state(id, GameState::default())
    }

    pub fn new_with_profile(player_profile: PlayerProfile) -> Self {
        Self::new_with_profile_and_state(player_profile, GameState::default())
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
            10
        );
    }
}
