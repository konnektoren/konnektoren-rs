//! What a performance (0–100 %) means: not passed, passed, good, mastered, and the
//! stars for each.
//!
//! The default follows common practice: **passed from 60 %** (German exams such as the
//! Goethe-Zertifikat modules, where 70 % is "befriedigend") and **mastered from 80 %**
//! (mastery learning), with **good from 70 %** in between. Apps that judge differently
//! define their own scale once, e.g.
//!
//! ```
//! use konnektoren_core::challenges::PerformanceScale;
//!
//! pub const SCALE: PerformanceScale = PerformanceScale::DEFAULT.with_passed(50);
//! assert!(SCALE.is_passed(55));
//! ```
//!
//! and pass it wherever they work with results.
use serde::{Deserialize, Serialize};

/// Thresholds in percent. Keep `passed_from <= good_from <= mastered_from`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PerformanceScale {
    /// Passed, one star, from here; below is not passed.
    pub passed_from: u32,
    /// Good, two stars, from here.
    pub good_from: u32,
    /// Mastered, three stars, from here. Counts as successful in analytics.
    pub mastered_from: u32,
}

/// The result of one performance on a [`PerformanceScale`]; ordered from worst to best.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PerformanceBand {
    /// Below passed: no star, worth practising.
    NotPassed,
    /// One star.
    Passed,
    /// Two stars.
    Good,
    /// Three stars.
    Mastered,
}

impl PerformanceBand {
    /// No star when not passed, then one, two and three.
    pub const fn stars(self) -> u32 {
        match self {
            Self::NotPassed => 0,
            Self::Passed => 1,
            Self::Good => 2,
            Self::Mastered => 3,
        }
    }
}

impl PerformanceScale {
    /// Passed from 60 %, good from 70 %, mastered from 80 %.
    pub const DEFAULT: Self = Self {
        passed_from: 60,
        good_from: 70,
        mastered_from: 80,
    };

    pub const fn with_passed(mut self, percent: u32) -> Self {
        self.passed_from = percent;
        self
    }

    pub const fn with_good(mut self, percent: u32) -> Self {
        self.good_from = percent;
        self
    }

    pub const fn with_mastered(mut self, percent: u32) -> Self {
        self.mastered_from = percent;
        self
    }

    pub const fn is_passed(&self, performance: u32) -> bool {
        performance >= self.passed_from
    }

    pub const fn is_mastered(&self, performance: u32) -> bool {
        performance >= self.mastered_from
    }

    pub const fn band(&self, performance: u32) -> PerformanceBand {
        if performance >= self.mastered_from {
            PerformanceBand::Mastered
        } else if performance >= self.good_from {
            PerformanceBand::Good
        } else if performance >= self.passed_from {
            PerformanceBand::Passed
        } else {
            PerformanceBand::NotPassed
        }
    }

    /// Three stars when mastered, two when good, one when passed, else none.
    pub const fn stars(&self, performance: u32) -> u32 {
        self.band(performance).stars()
    }
}

impl Default for PerformanceScale {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_passes_from_60_is_good_from_70_and_masters_from_80() {
        let scale = PerformanceScale::default();
        assert_eq!(scale, PerformanceScale::DEFAULT);
        assert!(!scale.is_passed(59) && scale.is_passed(60));
        assert!(!scale.is_mastered(79) && scale.is_mastered(80));
        let bands: Vec<_> = [0, 59, 60, 69, 70, 79, 80, 100]
            .into_iter()
            .map(|performance| scale.band(performance))
            .collect();
        use PerformanceBand::*;
        assert_eq!(
            bands,
            [
                NotPassed, NotPassed, Passed, Passed, Good, Good, Mastered, Mastered
            ]
        );
    }

    #[test]
    fn stars_follow_the_bands() {
        let scale = PerformanceScale::DEFAULT;
        let stars: Vec<_> = [0, 59, 60, 69, 70, 79, 80, 100]
            .into_iter()
            .map(|performance| scale.stars(performance))
            .collect();
        assert_eq!(stars, [0, 0, 1, 1, 2, 2, 3, 3]);
    }

    #[test]
    fn apps_override_single_thresholds_in_a_const() {
        const SCALE: PerformanceScale = PerformanceScale::DEFAULT
            .with_passed(50)
            .with_good(65)
            .with_mastered(90);
        assert_eq!(SCALE.stars(49), 0);
        assert_eq!(SCALE.stars(50), 1);
        assert_eq!(SCALE.stars(65), 2);
        assert_eq!(SCALE.stars(89), 2);
        assert_eq!(SCALE.stars(90), 3);
        assert!(PerformanceBand::Mastered > PerformanceBand::NotPassed);
    }
}
