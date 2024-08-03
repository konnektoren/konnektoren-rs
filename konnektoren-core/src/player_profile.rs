use serde::{Deserialize, Serialize};

use crate::Xp;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerProfile {
    pub id: String,
    pub name: String,
    pub xp: Xp,
}

impl PlayerProfile {
    pub fn new(id: String) -> Self {
        PlayerProfile {
            id,
            ..Default::default()
        }
    }
}

impl Default for PlayerProfile {
    fn default() -> Self {
        let mut generator = names::Generator::default();
        let name = generator.next().unwrap();

        PlayerProfile {
            id: "".to_string(),
            name,
            xp: 0,
        }
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
