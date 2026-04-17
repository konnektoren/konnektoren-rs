#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A character taking part in a [`Dialog`](super::Dialog) challenge.
///
/// Every dialog has exactly two characters. Each turn in the dialog
/// references one by its `id`. The game engine uses `Speaker` to render
/// the correct name and avatar in the chat-bubble UI.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Speaker {
    /// Unique key used by [`DialogTurn::speaker`](super::turn::DialogTurn::speaker)
    /// to bind a turn to this character.
    pub id: String,

    /// Character name displayed above each chat bubble in the game UI.
    pub name: String,

    /// FontAwesome class (e.g. `"fa-solid fa-user"`) or image path used as
    /// the character's in-game avatar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

impl Default for Speaker {
    fn default() -> Self {
        Speaker {
            id: "speaker".to_string(),
            name: "Speaker".to_string(),
            icon: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_speaker_fields() {
        let speaker = Speaker::default();
        assert_eq!(speaker.id, "speaker");
        assert_eq!(speaker.name, "Speaker");
        assert_eq!(speaker.icon, None);
    }

    #[test]
    fn icon_skipped_when_none_in_json() {
        let speaker = Speaker::default();
        let json = serde_json::to_string(&speaker).expect("serialize");
        assert!(!json.contains("icon"));
    }

    #[test]
    fn icon_present_when_set_in_json() {
        let speaker = Speaker {
            id: "anna".to_string(),
            name: "Anna".to_string(),
            icon: Some("fa-solid fa-user".to_string()),
        };
        let json = serde_json::to_string(&speaker).expect("serialize");
        assert!(json.contains("icon"));
        assert!(json.contains("fa-solid fa-user"));
    }

    #[test]
    fn roundtrip_json_without_icon() {
        let original = Speaker::default();
        let json = serde_json::to_string(&original).expect("serialize");
        let restored: Speaker = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(original, restored);
    }

    #[test]
    fn roundtrip_json_with_icon() {
        let original = Speaker {
            id: "ben".to_string(),
            name: "Ben".to_string(),
            icon: Some("fa-solid fa-user-tie".to_string()),
        };
        let json = serde_json::to_string(&original).expect("serialize");
        let restored: Speaker = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(original, restored);
    }

    #[test]
    fn roundtrip_yaml() {
        let original = Speaker {
            id: "baecker".to_string(),
            name: "Bäcker".to_string(),
            icon: Some("fa-solid fa-bread-slice".to_string()),
        };
        let yaml = serde_yaml::to_string(&original).expect("serialize");
        let restored: Speaker = serde_yaml::from_str(&yaml).expect("deserialize");
        assert_eq!(original, restored);
    }

    #[test]
    fn parse_from_yaml_with_icon() {
        let yaml = r#"
id: "arzt"
name: "Arzt"
icon: "fa-solid fa-user-doctor"
"#;
        let speaker: Speaker = serde_yaml::from_str(yaml).expect("parse");
        assert_eq!(speaker.id, "arzt");
        assert_eq!(speaker.name, "Arzt");
        assert_eq!(speaker.icon, Some("fa-solid fa-user-doctor".to_string()));
    }

    #[test]
    fn parse_from_yaml_without_icon() {
        let yaml = r#"
id: "kunde"
name: "Kunde"
"#;
        let speaker: Speaker = serde_yaml::from_str(yaml).expect("parse");
        assert_eq!(speaker.id, "kunde");
        assert_eq!(speaker.name, "Kunde");
        assert_eq!(speaker.icon, None);
    }
}
