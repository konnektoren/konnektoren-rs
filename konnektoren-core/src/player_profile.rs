pub struct PlayerProfile {
    pub id: String,
    pub name: String,
}

impl PlayerProfile {
    pub fn new(id: String) -> Self {
        let name = "Anonymous".to_string();
        PlayerProfile { id, name }
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
