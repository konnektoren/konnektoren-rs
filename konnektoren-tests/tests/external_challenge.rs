use konnektoren_rs::core::challenges::{
    Challenge, ChallengeConfig, ChallengeHistory, Performance, Solvable, Timed,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ExternalChallenge;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ExternalResult(u32);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum MusicVariant {
    Melody,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum MusicChallenge {
    Melody { id: String, bpm: u32 },
}

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
    let mut history = ChallengeHistory::default();
    history.add_challenge(attempt.clone());
    history.add_challenge(attempt);
    assert_eq!(history.len(), 1);
    let restored: ChallengeHistory<Challenge<ExternalChallenge, ExternalResult, String>> =
        serde_yaml::from_str(&serde_yaml::to_string(&history)?)?;
    assert_eq!(restored, history);
    Ok(())
}

#[test]
fn external_challenge_and_variant_load_from_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let content: MusicChallenge = serde_yaml::from_str("!melody\nid: fais_dodo_a1\nbpm: 80\n")?;
    assert_eq!(
        content,
        MusicChallenge::Melody {
            id: "fais_dodo_a1".into(),
            bpm: 80
        }
    );
    assert_eq!(
        serde_yaml::from_str::<MusicChallenge>(&serde_yaml::to_string(&content)?)?,
        content
    );
    assert_eq!(
        serde_json::from_str::<MusicChallenge>(&serde_json::to_string(&content)?)?,
        content
    );

    let config: ChallengeConfig<MusicVariant> = serde_yaml::from_str(
        "id: lullaby\nname: Lullaby\ndescription: Play\nchallenge: fais_dodo_a1\nvariant: melody\ntasks: 1\nunlock_points: 0\n",
    )?;
    assert_eq!(config.variant, Some(MusicVariant::Melody));
    assert_eq!(
        serde_yaml::from_str::<ChallengeConfig<MusicVariant>>(&serde_yaml::to_string(&config)?)?,
        config
    );
    Ok(())
}
