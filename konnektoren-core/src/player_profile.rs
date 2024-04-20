use serde::{Deserialize, Serialize};

use crate::Xp;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerProfile {
    pub id: String,
    pub name: String,
    pub xp: Xp,
}

impl PlayerProfile {
    pub fn new(id: String) -> Self {
        let name = "Anonymous".to_string();
        let xp = 0;
        PlayerProfile { id, name, xp }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_profile() {
        let id = "123".to_string();
        let profile = PlayerProfile::new(id.clone());
        assert_eq!(profile.id, id);
        assert_eq!(profile.name, "Anonymous");
    }
}
