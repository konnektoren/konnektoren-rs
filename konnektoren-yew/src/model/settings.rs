use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub language: String,
    pub music_volume: f32,
    pub sound_volume: f32,
    pub id: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            language: "en".to_string(),
            music_volume: 0.4,
            sound_volume: 0.8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings() {
        let settings = Settings::default();
        assert_eq!(settings.language, "en");
        assert_eq!(settings.music_volume, 0.4);
        assert_eq!(settings.sound_volume, 0.8);
    }

    #[test]
    fn ser_de_settings() {
        let settings = Settings::default();
        let json = serde_json::to_string(&settings).unwrap();
        let settings2: Settings = serde_json::from_str(&json).unwrap();
        assert_eq!(settings, settings2);
    }
}