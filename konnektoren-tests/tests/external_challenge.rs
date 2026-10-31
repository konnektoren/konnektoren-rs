use konnektoren_rs::core::challenges::{Performance, Solvable};

struct ExternalChallenge;
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
