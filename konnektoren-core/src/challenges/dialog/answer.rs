use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A player's choice at one interactive turn of a [`Dialog`](super::Dialog) challenge.
///
/// Recorded each time the player picks an option during a [`DialogQuiz`] run.
/// A completed [`Dialog`] challenge accumulates one [`DialogAnswer`] per
/// interactive turn the player acted on.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct DialogAnswer {
    /// Position of the turn in [`Dialog::turns`](super::dialog::Dialog::turns)
    /// that the player responded to.
    pub turn_index: usize,
    /// Index into [`DialogTurn::options`](super::turn::DialogTurn::options)
    /// that the player selected.
    pub selected_option: usize,
}

impl Default for DialogAnswer {
    fn default() -> Self {
        DialogAnswer {
            turn_index: 0,
            selected_option: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_answer() {
        let answer = DialogAnswer::default();
        assert_eq!(answer.turn_index, 0);
        assert_eq!(answer.selected_option, 0);
    }

    #[test]
    fn roundtrip_json() {
        let answer = DialogAnswer {
            turn_index: 3,
            selected_option: 1,
        };
        let json = serde_json::to_string(&answer).expect("serialize");
        let restored: DialogAnswer = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(answer, restored);
    }

    #[test]
    fn roundtrip_yaml() {
        let answer = DialogAnswer {
            turn_index: 2,
            selected_option: 0,
        };
        let yaml = serde_yaml::to_string(&answer).expect("serialize");
        let restored: DialogAnswer = serde_yaml::from_str(&yaml).expect("deserialize");
        assert_eq!(answer, restored);
    }

    #[test]
    fn fields_are_preserved() {
        let answer = DialogAnswer {
            turn_index: 7,
            selected_option: 2,
        };
        assert_eq!(answer.turn_index, 7);
        assert_eq!(answer.selected_option, 2);
    }
}
