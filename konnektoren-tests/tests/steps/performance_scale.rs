use crate::BddWorld;
use cucumber::{given, then, when};
use konnektoren_rs::achievements::{
    AchievementEvaluator, GameStatistics, MasteredChallenges, PassedChallenges,
};
use konnektoren_rs::core::analytics::Metric;
use konnektoren_rs::core::analytics::metrics::SuccessRateMetric;
use konnektoren_rs::core::challenges::PerformanceScale;

#[given(expr = "the default performance scale")]
async fn default_scale(world: &mut BddWorld) {
    world.scale = PerformanceScale::DEFAULT;
}

#[given(expr = "an app scale that passes from {int}%")]
async fn app_scale_passes_from(world: &mut BddWorld, percent: u32) {
    world.scale = PerformanceScale::DEFAULT.with_passed(percent);
}

#[given(expr = "an app scale that masters from {int}%")]
async fn app_scale_masters_from(world: &mut BddWorld, percent: u32) {
    world.scale = PerformanceScale::DEFAULT.with_mastered(percent);
}

#[when(expr = "a performance of {int}% is judged")]
async fn performance_is_judged(world: &mut BddWorld, performance: u32) {
    world.judged_performance = Some(performance);
}

fn judged(world: &BddWorld) -> u32 {
    world
        .judged_performance
        .expect("a performance should have been judged")
}

#[then(expr = "the performance band should be {string}")]
async fn band_should_be(world: &mut BddWorld, band: String) {
    let actual = world.scale.band(judged(world));
    assert_eq!(format!("{actual:?}"), band);
}

#[then(expr = "the performance should earn {int} stars")]
async fn should_earn_stars(world: &mut BddWorld, stars: u32) {
    assert_eq!(world.scale.stars(judged(world)), stars);
}

#[then(expr = "the XP reward for the last attempt should be {int}")]
async fn xp_reward_should_be(world: &mut BddWorld, xp: u32) {
    let attempt = world
        .game
        .challenge_history
        .challenges
        .last()
        .expect("an attempt should exist");
    assert_eq!(world.game.calculate_xp_reward(attempt), xp);
}

#[then(expr = "the success rate should be {int}%")]
async fn success_rate_should_be(world: &mut BddWorld, rate: u32) {
    let metric =
        SuccessRateMetric::new(world.game.challenge_history.clone()).with_scale(world.scale);
    assert_eq!(metric.value(), rate as f64);
}

#[then(expr = "the user should have {int} passed and {int} mastered challenges")]
async fn passed_and_mastered(world: &mut BddWorld, passed: u32, mastered: u32) {
    let statistics = GameStatistics::new(&world.game).with_scale(world.scale);
    assert_eq!(statistics.passed_challenges(), passed);
    assert_eq!(statistics.mastered_challenges(), mastered);
}

#[given(expr = "an achievement with the condition {string}")]
async fn achievement_with_condition(world: &mut BddWorld, condition: String) {
    world.achievement_yaml = Some(format!(
        "achievements:\n  - id: scaled\n    name: Scaled\n    description: Scaled achievement\n    icon: ⭐\n    condition: \"{condition}\"\n"
    ));
}

fn unlocked(world: &BddWorld) -> bool {
    let yaml = world
        .achievement_yaml
        .as_deref()
        .expect("an achievement should be defined");
    let evaluator = AchievementEvaluator::new(yaml)
        .expect("valid achievement yaml")
        .with_scale(world.scale);
    !evaluator.evaluate(&world.game).is_empty()
}

#[then(expr = "the achievement should be unlocked")]
async fn achievement_unlocked(world: &mut BddWorld) {
    assert!(unlocked(world), "the achievement should be unlocked");
}

#[then(expr = "the achievement should not be unlocked")]
async fn achievement_locked(world: &mut BddWorld) {
    assert!(!unlocked(world), "the achievement should stay locked");
}
