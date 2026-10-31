use konnektoren_core::game::GamePath;

/// References needed to preload a factory, independent of level presentation data.
pub trait LevelDefinition {
    fn id(&self) -> &str;
    /// Pairs of presentation entry ID and referenced content ID, in level order.
    fn challenge_references(&self) -> impl Iterator<Item = (&str, &str)>;
}

impl LevelDefinition for GamePath {
    fn id(&self) -> &str {
        &self.id
    }
    fn challenge_references(&self) -> impl Iterator<Item = (&str, &str)> {
        self.challenges
            .iter()
            .map(|config| (config.id.as_str(), config.challenge.as_str()))
    }
}
