use serde::{Deserialize, Serialize};

use crate::player_profile::PlayerProfile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub player_profile: PlayerProfile,
}

impl Session {
    pub fn new(id: String) -> Self {
        let player_profile = PlayerProfile::new(id.clone());
        Session { id, player_profile }
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
        assert_eq!(session.player_profile.name, "Anonymous");
    }
}
