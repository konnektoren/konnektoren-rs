use konnektoren_rs::core::challenges::Performance;

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
