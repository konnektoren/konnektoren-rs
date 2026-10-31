use konnektoren_rs::core::challenges::{Challenge, Performance, Solvable, Timed};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ExternalChallenge;
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ExternalResult(u32);

impl Performance<ExternalResult> for ExternalChallenge {
    fn performance(&self, result: &ExternalResult) -> u32 {
        result.0
    }
}

#[test]
fn external_challenges_share_core_star_thresholds() {
    let challenge = ExternalChallenge;
    for (score, stars) in [(0, 0), (39, 0), (40, 1), (60, 2), (80, 3), (100, 3)] {
        assert_eq!(challenge.stars(&ExternalResult(score)), stars);
    }
}

struct NoteChallenge {
    expected: u8,
    played: Option<u8>,
}

#[derive(Debug, PartialEq)]
struct InvalidTask;

impl Solvable<u8, InvalidTask> for NoteChallenge {
    fn solve(&mut self, input: u8, task_index: usize) -> Result<bool, InvalidTask> {
        if task_index != 0 {
            return Err(InvalidTask);
        }
        self.played = Some(input);
        Ok(input == self.expected)
    }
}

#[test]
fn external_challenges_accept_their_own_input_and_error() {
    let mut challenge = NoteChallenge {
        expected: 60,
        played: None,
    };
    assert_eq!(challenge.solve(61, 0), Ok(false));
    assert_eq!(challenge.solve(60, 0), Ok(true));
    assert_eq!(challenge.played, Some(60));
    assert_eq!(challenge.solve(62, 1), Err(InvalidTask));
    assert_eq!(challenge.played, Some(60));
}

#[test]
fn external_attempt_round_trips_with_timing_and_performance()
-> Result<(), Box<dyn std::error::Error>> {
    let mut attempt = Challenge {
        challenge_type: ExternalChallenge,
        challenge_config: "music".to_owned(),
        challenge_result: ExternalResult(80),
        start_time: None,
        end_time: None,
    };
    attempt.start();
    attempt.update_end_time();
    assert!(attempt.elapsed_time().is_some());
    assert_eq!(attempt.stars(&attempt.challenge_result), 3);
    let yaml = serde_yaml::to_string(&attempt)?;
    let restored: Challenge<ExternalChallenge, ExternalResult, String> =
        serde_yaml::from_str(&yaml)?;
    assert_eq!(restored, attempt);
    let json = serde_json::to_string(&attempt)?;
    let restored: Challenge<ExternalChallenge, ExternalResult, String> =
        serde_json::from_str(&json)?;
    assert_eq!(restored, attempt);
    Ok(())
}
