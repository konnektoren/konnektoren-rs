use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A single utterance in a [`Dialog`](super::dialog::Dialog) challenge.
///
/// A turn is an *interactive turn* when both `options` and `correct_option` are
/// present.  In **Observer** mode those fields are ignored and the player watches
/// the exchange unfold.  In **Quiz** mode the player must pick the correct
/// option to score points before the line is revealed.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct DialogTurn {
    /// References `Speaker.id` — identifies which character delivers this line.
    pub speaker: String,

    /// The correct line of dialog for this turn.
    ///
    /// Always present.  Concealed from the player until they commit an answer
    /// in Quiz mode; shown immediately in Observer mode.
    pub text: String,

    /// Optional audio file reference (URL or asset path) played when the turn
    /// is revealed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<String>,

    /// Candidate lines the player may choose from in Quiz mode.
    ///
    /// When present this turn becomes interactive — the player selects one
    /// entry and their choice is evaluated against
    /// [`correct_option`](Self::correct_option).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,

    /// Index into [`options`](Self::options) that awards the point.
    ///
    /// **Must** be present whenever `options` is present, and must be a
    /// valid index into that vector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option: Option<usize>,
}

impl Default for DialogTurn {
    fn default() -> Self {
        DialogTurn {
            speaker: "speaker".to_string(),
            text: "Hello!".to_string(),
            audio: None,
            options: None,
            correct_option: None,
        }
    }
}

impl DialogTurn {
    /// Returns `true` when this turn requires a player choice in Quiz mode.
    ///
    /// A turn is interactive when both `options` **and** `correct_option` are
    /// set.  A turn that only carries `options` but no `correct_option` is
    /// displayed as flavour text and does not affect the score.
    pub fn is_quiz_turn(&self) -> bool {
        self.options.is_some() && self.correct_option.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Default ──────────────────────────────────────────────────────────────

    #[test]
    fn default_turn_has_expected_fields() {
        let turn = DialogTurn::default();
        assert_eq!(turn.speaker, "speaker");
        assert_eq!(turn.text, "Hello!");
        assert!(turn.audio.is_none());
        assert!(turn.options.is_none());
        assert!(turn.correct_option.is_none());
    }

    // ── is_quiz_turn ─────────────────────────────────────────────────────────

    #[test]
    fn is_quiz_turn_when_options_and_correct_option_present() {
        let turn = DialogTurn {
            speaker: "a".to_string(),
            text: "Guten Morgen!".to_string(),
            audio: None,
            options: Some(vec!["Guten Morgen!".to_string(), "Gute Nacht!".to_string()]),
            correct_option: Some(0),
        };
        assert!(turn.is_quiz_turn());
    }

    #[test]
    fn not_quiz_turn_without_options() {
        let turn = DialogTurn::default();
        assert!(!turn.is_quiz_turn());
    }

    #[test]
    fn not_quiz_turn_with_options_but_no_correct_option() {
        let turn = DialogTurn {
            speaker: "a".to_string(),
            text: "Hallo!".to_string(),
            audio: None,
            options: Some(vec!["Hallo!".to_string()]),
            correct_option: None,
        };
        assert!(!turn.is_quiz_turn());
    }

    #[test]
    fn not_quiz_turn_with_correct_option_but_no_options() {
        let turn = DialogTurn {
            speaker: "a".to_string(),
            text: "Hallo!".to_string(),
            audio: None,
            options: None,
            correct_option: Some(0),
        };
        assert!(!turn.is_quiz_turn());
    }

    // ── Serialisation ─────────────────────────────────────────────────────────

    #[test]
    fn optional_fields_absent_from_json_when_none() {
        let turn = DialogTurn::default();
        let json = serde_json::to_string(&turn).expect("serialize");
        assert!(!json.contains("audio"));
        assert!(!json.contains("options"));
        assert!(!json.contains("correct_option"));
    }

    #[test]
    fn optional_fields_present_in_json_when_some() {
        let turn = DialogTurn {
            speaker: "a".to_string(),
            text: "Ja, bitte.".to_string(),
            audio: Some("audio/ja_bitte.mp3".to_string()),
            options: Some(vec!["Ja, bitte.".to_string(), "Nein, danke.".to_string()]),
            correct_option: Some(0),
        };
        let json = serde_json::to_string(&turn).expect("serialize");
        assert!(json.contains("audio"));
        assert!(json.contains("options"));
        assert!(json.contains("correct_option"));
    }

    #[test]
    fn roundtrip_json_plain_turn() {
        let original = DialogTurn::default();
        let json = serde_json::to_string(&original).expect("serialize");
        let restored: DialogTurn = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(original, restored);
    }

    #[test]
    fn roundtrip_json_quiz_turn() {
        let original = DialogTurn {
            speaker: "kunde".to_string(),
            text: "Ich hätte gerne vier Brötchen, bitte.".to_string(),
            audio: None,
            options: Some(vec![
                "Ich hätte gerne vier Brötchen, bitte.".to_string(),
                "Guten Abend! Vier Brötchen geben.".to_string(),
                "Hallo! Was haben Sie?".to_string(),
            ]),
            correct_option: Some(0),
        };
        let json = serde_json::to_string(&original).expect("serialize");
        let restored: DialogTurn = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(original, restored);
    }

    #[test]
    fn roundtrip_yaml_plain_turn() {
        let original = DialogTurn::default();
        let yaml = serde_yaml::to_string(&original).expect("serialize");
        let restored: DialogTurn = serde_yaml::from_str(&yaml).expect("deserialize");
        assert_eq!(original, restored);
    }

    #[test]
    fn roundtrip_yaml_quiz_turn() {
        let yaml = r#"
speaker: "ben"
text: "Guten Morgen, Anna! Wie geht's?"
options:
  - "Guten Morgen, Anna! Wie geht's?"
  - "Gute Nacht, Anna!"
  - "Auf Wiedersehen!"
correct_option: 0
"#;
        let turn: DialogTurn = serde_yaml::from_str(yaml).expect("deserialize");
        assert_eq!(turn.speaker, "ben");
        assert_eq!(turn.correct_option, Some(0));
        assert!(turn.is_quiz_turn());
        assert_eq!(turn.options.as_ref().unwrap().len(), 3);

        // Re-serialise and parse again
        let yaml2 = serde_yaml::to_string(&turn).expect("serialize");
        let turn2: DialogTurn = serde_yaml::from_str(&yaml2).expect("deserialize");
        assert_eq!(turn, turn2);
    }

    // ── Clone & PartialEq ────────────────────────────────────────────────────

    #[test]
    fn clone_produces_equal_value() {
        let original = DialogTurn {
            speaker: "anna".to_string(),
            text: "Tschüss!".to_string(),
            audio: Some("audio/tschuess.mp3".to_string()),
            options: None,
            correct_option: None,
        };
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn different_speakers_are_not_equal() {
        let a = DialogTurn {
            speaker: "anna".to_string(),
            ..DialogTurn::default()
        };
        let b = DialogTurn {
            speaker: "ben".to_string(),
            ..DialogTurn::default()
        };
        assert_ne!(a, b);
    }
}
