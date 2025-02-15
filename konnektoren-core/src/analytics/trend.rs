use strum_macros::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, Display)]
pub enum Trend {
    Improving,
    #[default]
    Stable,
    Declining,
}

impl Trend {
    const EPSILON: f64 = 0.0001;

    pub fn from_value(value: f64) -> Self {
        if value > Self::EPSILON {
            Trend::Improving
        } else if value < -Self::EPSILON {
            Trend::Declining
        } else {
            Trend::Stable
        }
    }
}

impl PartialOrd for Trend {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            // Improving is greater than everything except itself
            (Trend::Improving, Trend::Improving) => Some(std::cmp::Ordering::Equal),
            (Trend::Improving, _) => Some(std::cmp::Ordering::Greater),

            // Stable is less than Improving, greater than Declining
            (Trend::Stable, Trend::Improving) => Some(std::cmp::Ordering::Less),
            (Trend::Stable, Trend::Stable) => Some(std::cmp::Ordering::Equal),
            (Trend::Stable, Trend::Declining) => Some(std::cmp::Ordering::Greater),

            // Declining is less than everything except itself
            (Trend::Declining, Trend::Declining) => Some(std::cmp::Ordering::Equal),
            (Trend::Declining, _) => Some(std::cmp::Ordering::Less),
        }
    }
}

impl Ord for Trend {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trend_from_value() {
        assert_eq!(Trend::from_value(1.5), Trend::Improving);
        assert_eq!(Trend::from_value(-1.5), Trend::Declining);
        assert_eq!(Trend::from_value(0.0), Trend::Stable);
        assert_eq!(Trend::from_value(0.001), Trend::Improving);
        assert_eq!(Trend::from_value(-0.001), Trend::Declining);
    }

    #[test]
    fn test_trend_ordering() {
        // Test Improving comparisons
        assert!(Trend::Improving > Trend::Stable);
        assert!(Trend::Improving > Trend::Declining);
        assert!(Trend::Improving == Trend::Improving);

        // Test Stable comparisons
        assert!(Trend::Stable < Trend::Improving);
        assert!(Trend::Stable == Trend::Stable);
        assert!(Trend::Stable > Trend::Declining);

        // Test Declining comparisons
        assert!(Trend::Declining < Trend::Improving);
        assert!(Trend::Declining < Trend::Stable);
        assert!(Trend::Declining == Trend::Declining);
    }

    #[test]
    fn test_trend_default() {
        assert_eq!(Trend::default(), Trend::Stable);
    }

    #[test]
    fn test_trend_sorting() {
        let mut trends = vec![
            Trend::Declining,
            Trend::Improving,
            Trend::Stable,
            Trend::Improving,
            Trend::Declining,
        ];
        trends.sort();

        assert_eq!(
            trends,
            vec![
                Trend::Declining,
                Trend::Declining,
                Trend::Stable,
                Trend::Improving,
                Trend::Improving,
            ]
        );
    }

    #[test]
    fn test_trend_clone_and_copy() {
        let trend = Trend::Improving;
        let cloned = trend.clone();
        assert_eq!(trend, cloned);

        let copied = trend;
        assert_eq!(trend, copied);
    }

    #[test]
    fn test_trend_debug_format() {
        assert_eq!(format!("{:?}", Trend::Improving), "Improving");
        assert_eq!(format!("{:?}", Trend::Stable), "Stable");
        assert_eq!(format!("{:?}", Trend::Declining), "Declining");
    }

    #[test]
    fn test_trend_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(Trend::Improving);
        set.insert(Trend::Stable);
        set.insert(Trend::Declining);
        set.insert(Trend::Improving); // Duplicate

        assert_eq!(set.len(), 3);
        assert!(set.contains(&Trend::Improving));
        assert!(set.contains(&Trend::Stable));
        assert!(set.contains(&Trend::Declining));
    }

    #[test]
    fn test_trend_partial_ord() {
        // Test all possible combinations
        let trends = [Trend::Improving, Trend::Stable, Trend::Declining];

        for &t1 in &trends {
            for &t2 in &trends {
                // Ensure partial_cmp always returns Some value
                assert!(t1.partial_cmp(&t2).is_some());

                // Test reflexivity
                if t1 == t2 {
                    assert_eq!(t1.partial_cmp(&t2), Some(std::cmp::Ordering::Equal));
                }

                // Test transitivity
                if t1 > t2 {
                    assert_eq!(t1.partial_cmp(&t2), Some(std::cmp::Ordering::Greater));
                } else if t1 < t2 {
                    assert_eq!(t1.partial_cmp(&t2), Some(std::cmp::Ordering::Less));
                }
            }
        }
    }
}
