use konnektoren_rs::core::challenges::task_pattern::TaskPattern;
use konnektoren_rs::core::challenges::{
    Base64Serializable, Challenge, ChallengeConfig, ChallengeDefinition, ChallengeFactory,
    ChallengeHistory, Performance, Solvable, Timed,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ExternalChallenge;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ExternalResult(u32);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum ExternalVariant {
    Example,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum ExternalContent {
    Example { id: String, value: u32 },
}

impl Base64Serializable for ExternalContent {}

#[test]
fn external_factory_imports_and_exports_content() -> Result<(), Box<dyn std::error::Error>> {
    let content = ExternalContent::Example {
        id: "external".into(),
        value: 42,
    };
    let mut factory = ChallengeFactory::<ExternalContent>::empty();
    factory.add_challenge_from_base64(&content.to_base64()?)?;
    assert_eq!(factory.challenge_types, vec![content.clone()]);
    let encoded = factory.export_challenge_to_base64("external")?;
    assert_eq!(ExternalContent::from_base64(&encoded)?, content);
    assert!(factory.export_challenge_to_base64("missing").is_err());
    assert!(factory.add_challenge_from_base64("invalid!").is_err());
    assert_eq!(factory.challenge_types.len(), 1);
    Ok(())
}

impl ChallengeDefinition for ExternalContent {
    type Result = Vec<u8>;
    fn id(&self) -> &str {
        match self {
            Self::Example { id, .. } => id,
        }
    }

    fn of_tasks(&self, _tasks: &TaskPattern) -> Self {
        self.clone()
    }
}

struct TaskSequence(Vec<u8>);

impl ChallengeDefinition for TaskSequence {
    type Result = Vec<Option<u8>>;
    fn id(&self) -> &str {
        "tasks"
    }

    fn of_tasks(&self, tasks: &TaskPattern) -> Self {
        Self(tasks.select_items(&self.0))
    }
}

#[test]
fn factory_selects_tasks_without_creating_a_result() -> Result<(), Box<dyn std::error::Error>> {
    let factory = ChallengeFactory {
        challenge_types: vec![TaskSequence(vec![60, 62, 64])],
    };
    let config = ChallengeConfig {
        challenge: "tasks".into(),
        tasks: TaskPattern::Exact(2),
        ..ChallengeConfig::default()
    };
    let challenge = factory.create_challenge(&config)?;
    assert_eq!(challenge.challenge_type.0, [60, 62]);
    assert!(challenge.challenge_result.is_none());
    assert_eq!(factory.challenge_types[0].0, [60, 62, 64]);
    Ok(())
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

struct AnswerChallenge {
    expected: u8,
    answer: Option<u8>,
}

#[derive(Debug, PartialEq)]
struct InvalidTask;

impl Solvable<u8, InvalidTask> for AnswerChallenge {
    fn solve(&mut self, input: u8, task_index: usize) -> Result<bool, InvalidTask> {
        if task_index != 0 {
            return Err(InvalidTask);
        }
        self.answer = Some(input);
        Ok(input == self.expected)
    }
}

#[test]
fn external_challenges_accept_their_own_input_and_error() {
    let mut challenge = AnswerChallenge {
        expected: 60,
        answer: None,
    };
    assert_eq!(challenge.solve(61, 0), Ok(false));
    assert_eq!(challenge.solve(60, 0), Ok(true));
    assert_eq!(challenge.answer, Some(60));
    assert_eq!(challenge.solve(62, 1), Err(InvalidTask));
    assert_eq!(challenge.answer, Some(60));
}

#[test]
fn external_attempt_round_trips_with_timing_and_performance()
-> Result<(), Box<dyn std::error::Error>> {
    let mut attempt = Challenge {
        challenge_type: ExternalChallenge,
        challenge_config: "external".to_owned(),
        challenge_result: Some(ExternalResult(80)),
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
    let content: ExternalContent = serde_yaml::from_str("!example\nid: example-1\nvalue: 80\n")?;
    assert_eq!(
        content,
        ExternalContent::Example {
            id: "example-1".into(),
            value: 80
        }
    );
    assert_eq!(
        serde_yaml::from_str::<ExternalContent>(&serde_yaml::to_string(&content)?)?,
        content
    );
    assert_eq!(
        serde_json::from_str::<ExternalContent>(&serde_json::to_string(&content)?)?,
        content
    );

    let config: ChallengeConfig<ExternalVariant> = serde_yaml::from_str(
        "id: example-run\nname: Example\ndescription: Play\nchallenge: example-1\nvariant: example\ntasks: 1\nunlock_points: 0\n",
    )?;
    assert_eq!(config.variant, Some(ExternalVariant::Example));
    assert_eq!(
        serde_yaml::from_str::<ChallengeConfig<ExternalVariant>>(&serde_yaml::to_string(&config)?)?,
        config
    );
    let factory = ChallengeFactory {
        challenge_types: vec![content.clone()],
    };
    let challenge = factory.create_challenge(&config)?;
    assert_eq!(challenge.challenge_type, content);
    assert!(challenge.challenge_result.is_none());
    assert_eq!(challenge.challenge_config, config);
    Ok(())
}

#[test]
fn nested_challenge_families_keep_each_result_type() -> Result<(), Box<dyn std::error::Error>> {
    use konnektoren_rs::core::challenges::{
        ChallengeFamily, ChallengeFamilyResult, ChallengeResult, ChallengeType,
    };
    type AppContent =
        ChallengeFamily<ChallengeType, ChallengeFamily<ExternalContent, TaskSequence>>;
    let core: AppContent = ChallengeFamily::First(ChallengeType::default());
    let external: AppContent =
        ChallengeFamily::Second(ChallengeFamily::Second(TaskSequence(vec![1, 2, 3])));
    let factory = ChallengeFactory {
        challenge_types: vec![core, external],
    };
    let config = ChallengeConfig {
        challenge: "tasks".into(),
        tasks: TaskPattern::Exact(2),
        ..ChallengeConfig::default()
    };
    let mut challenge = factory.create_challenge(&config)?;
    match &challenge.challenge_type {
        ChallengeFamily::Second(ChallengeFamily::Second(content)) => assert_eq!(content.0, [1, 2]),
        _ => panic!("expected the external task sequence"),
    }
    challenge.challenge_result = Some(ChallengeFamilyResult::Second(
        ChallengeFamilyResult::Second(vec![Some(1), None]),
    ));
    assert!(matches!(
        challenge.challenge_result,
        Some(ChallengeFamilyResult::Second(
            ChallengeFamilyResult::Second(_)
        ))
    ));
    let core_result: <AppContent as ChallengeDefinition>::Result =
        ChallengeFamilyResult::First(ChallengeResult::default());
    assert!(matches!(
        core_result,
        ChallengeFamilyResult::First(ChallengeResult::MultipleChoice(_))
    ));
    Ok(())
}

impl Performance<Vec<u8>> for ExternalContent {
    fn performance(&self, result: &Vec<u8>) -> u32 {
        if result.is_empty() { 0 } else { 100 }
    }
}

impl Performance<Vec<Option<u8>>> for TaskSequence {
    fn performance(&self, result: &Vec<Option<u8>>) -> u32 {
        if result.iter().flatten().eq(self.0.iter()) {
            100
        } else {
            0
        }
    }
}

#[test]
fn family_scoring_checks_each_nested_branch() -> Result<(), Box<dyn std::error::Error>> {
    use konnektoren_rs::core::challenges::{
        ChallengeFamily, ChallengeFamilyResult, ChallengeResult, ChallengeType,
        FamilyResultMismatch,
    };
    type AppContent =
        ChallengeFamily<ChallengeType, ChallengeFamily<ExternalContent, TaskSequence>>;
    let challenge: AppContent =
        ChallengeFamily::Second(ChallengeFamily::Second(TaskSequence(vec![1, 2])));
    let result =
        ChallengeFamilyResult::Second(ChallengeFamilyResult::Second(vec![Some(1), Some(2)]));
    let score = challenge.try_with_result(
        &result,
        |core, answers| Ok(core.performance(answers)),
        |group, answers| group.try_performance(answers),
    )??;
    assert_eq!(score, 100);

    let wrong_outer = ChallengeFamilyResult::First(ChallengeResult::default());
    assert_eq!(
        challenge.try_with_result(&wrong_outer, |_, _| (), |_, _| ()),
        Err(FamilyResultMismatch)
    );
    let wrong_inner = ChallengeFamilyResult::Second(ChallengeFamilyResult::First(vec![1]));
    let inner_score = challenge.try_with_result(
        &wrong_inner,
        |core, answers| Ok(core.performance(answers)),
        |group, answers| group.try_performance(answers),
    )?;
    assert_eq!(inner_score, Err(FamilyResultMismatch));
    Ok(())
}

#[test]
fn flat_app_types_load_legacy_history_and_round_trip_mixed_attempts()
-> Result<(), Box<dyn std::error::Error>> {
    use konnektoren_rs::core::challenges::{MultipleChoice, MultipleChoiceOption};

    // An app-owned enum describes actual persisted types, not family positions.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Example {
        id: String,
        value: u32,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    enum FlatContent {
        MultipleChoice(MultipleChoice),
        Example(Example),
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    enum FlatResult {
        MultipleChoice(Vec<MultipleChoiceOption>),
        Example(Vec<u8>),
    }
    type Attempt = Challenge<FlatContent, FlatResult>;

    let legacy_json: serde_json::Value = serde_json::from_str(include_str!(
        "../../konnektoren-core/tests/fixtures/legacy-session.json"
    ))?;
    let original_history = legacy_json["game_state"]["game"]["challenge_history"].clone();
    let mut history: ChallengeHistory<Attempt> = serde_json::from_value(original_history.clone())?;
    assert_eq!(serde_json::to_value(&history)?, original_history);
    let legacy_yaml: serde_yaml::Value = serde_yaml::from_str(include_str!(
        "../../konnektoren-core/tests/fixtures/legacy-session.yml"
    ))?;
    let yaml_history: ChallengeHistory<Attempt> =
        serde_yaml::from_value(legacy_yaml["game_state"]["game"]["challenge_history"].clone())?;
    assert_eq!(yaml_history, history);

    history.add_challenge(Challenge {
        challenge_type: FlatContent::Example(Example {
            id: "external".into(),
            value: 42,
        }),
        challenge_config: ChallengeConfig::default(),
        challenge_result: Some(FlatResult::Example(vec![1, 2])),
        start_time: None,
        end_time: None,
    });
    let json = serde_json::to_string(&history)?;
    let decoded: ChallengeHistory<Attempt> = serde_json::from_str(&json)?;
    assert_eq!(decoded, history);
    assert!(!json.contains("family_branch"));
    assert!(!json.contains("\"branch\""));
    let yaml = serde_yaml::to_string(&history)?;
    let decoded: ChallengeHistory<Attempt> = serde_yaml::from_str(&yaml)?;
    assert_eq!(decoded, history);
    assert!(yaml.contains("!multiple-choice"));
    assert!(yaml.contains("!example"));
    assert!(!yaml.contains("branch:"));
    let mut invalid: serde_json::Value = serde_json::from_str(&json)?;
    invalid["challenges"][1]["challenge_result"] = serde_json::json!({"Unknown": []});
    assert!(serde_json::from_value::<ChallengeHistory<Attempt>>(invalid).is_err());
    Ok(())
}
