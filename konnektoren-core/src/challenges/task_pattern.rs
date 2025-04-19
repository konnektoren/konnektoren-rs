use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq)]
pub enum TaskPattern {
    Exact(usize),
    Range(RangeInclusive<i32>),
    Random(usize, Option<RangeInclusive<i32>>),
}

impl TaskPattern {
    pub fn parse(s: &str) -> Result<Self, String> {
        if let Ok(n) = s.parse::<usize>() {
            return Ok(TaskPattern::Exact(n));
        }

        if s.contains(':') {
            let parts: Vec<&str> = s.split(':').collect();
            if parts.len() != 2 {
                return Err("Invalid random pattern".to_string());
            }
            let n = parts[0]
                .parse::<usize>()
                .map_err(|_| "Invalid number of tasks".to_string())?;
            if parts[1] == "random" {
                return Ok(TaskPattern::Random(n, None));
            }
            let range = Self::parse_range(parts[1])?;
            return Ok(TaskPattern::Random(n, Some(range)));
        }

        Self::parse_range(s).map(TaskPattern::Range)
    }

    fn parse_range(s: &str) -> Result<RangeInclusive<i32>, String> {
        let parts: Vec<&str> = s.split("..").collect();
        match parts.len() {
            1 => {
                let n = parts[0]
                    .parse::<i32>()
                    .map_err(|_| "Invalid range".to_string())?;
                Ok(n..=n)
            }
            2 => {
                let start = parts[0].parse::<i32>().unwrap_or(i32::MIN);
                let end = parts[1].parse::<i32>().unwrap_or(i32::MAX);
                if start > end {
                    return Err("Invalid range".to_string());
                }
                Ok(start..=end)
            }
            _ => Err("Invalid range format".to_string()),
        }
    }

    pub fn select_items<T: Clone>(&self, items: &[T]) -> Vec<T> {
        match self {
            TaskPattern::Exact(n) => items.iter().take(*n).cloned().collect(),
            TaskPattern::Range(range) => {
                let start = *range.start().max(&0) as usize;
                let end = *range.end().min(&(items.len() as i32 - 1)) as usize;
                items[start..=end].to_vec()
            }
            TaskPattern::Random(n, range_opt) => {
                let range = range_opt
                    .as_ref()
                    .map(|r| {
                        let start = *r.start().max(&0) as usize;
                        let end = *r.end().min(&(items.len() as i32 - 1)) as usize;
                        start..=end
                    })
                    .unwrap_or(0..=items.len() - 1);

                let mut rng = thread_rng();
                items[range]
                    .choose_multiple(&mut rng, *n)
                    .cloned()
                    .collect()
            }
        }
    }

    pub fn len(&self) -> usize {
        match self {
            TaskPattern::Exact(n) => *n,
            TaskPattern::Range(range) => {
                let start = *range.start().max(&0) as usize;
                let end = *range.end() as usize;
                end.saturating_sub(start) + 1
            }
            TaskPattern::Random(n, _) => *n,
        }
    }
}

impl fmt::Display for TaskPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskPattern::Exact(n) => write!(f, "{}", n),
            TaskPattern::Range(range) => {
                if *range.start() == i32::MIN {
                    write!(f, "..={}", range.end())
                } else if *range.end() == i32::MAX {
                    write!(f, "{}..=", range.start())
                } else {
                    write!(f, "{}..={}", range.start(), range.end())
                }
            }
            TaskPattern::Random(n, Some(range)) => {
                if *range.start() == i32::MIN {
                    write!(f, "{}:..={}", n, range.end())
                } else if *range.end() == i32::MAX {
                    write!(f, "{}:{}..=", n, range.start())
                } else {
                    write!(f, "{}:{}..={}", n, range.start(), range.end())
                }
            }
            TaskPattern::Random(n, None) => write!(f, "{}:random", n),
        }
    }
}

impl Serialize for TaskPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            TaskPattern::Exact(n) => serializer.serialize_str(&n.to_string()),
            TaskPattern::Range(range) => {
                serializer.serialize_str(&format!("{}..{}", range.start(), range.end()))
            }
            TaskPattern::Random(n, Some(range)) => {
                serializer.serialize_str(&format!("{}:{}..{}", n, range.start(), range.end()))
            }
            TaskPattern::Random(n, None) => serializer.serialize_str(&format!("{}:random", n)),
        }
    }
}

impl<'de> Deserialize<'de> for TaskPattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        TaskPattern::parse(&s).map_err(serde::de::Error::custom)
    }
}

impl From<usize> for TaskPattern {
    fn from(value: usize) -> Self {
        TaskPattern::Exact(value)
    }
}

impl From<String> for TaskPattern {
    fn from(s: String) -> Self {
        TaskPattern::parse(&s).unwrap_or(TaskPattern::Exact(0))
    }
}

impl<'a> From<&'a str> for TaskPattern {
    fn from(s: &'a str) -> Self {
        TaskPattern::parse(s).unwrap_or(TaskPattern::Exact(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_exact() {
        assert_eq!(TaskPattern::parse("10"), Ok(TaskPattern::Exact(10)));
    }

    #[test]
    fn test_parse_range() {
        assert_eq!(TaskPattern::parse("1..10"), Ok(TaskPattern::Range(1..=10)));
        assert_eq!(
            TaskPattern::parse("..10"),
            Ok(TaskPattern::Range(i32::MIN..=10))
        );
        assert_eq!(
            TaskPattern::parse("10.."),
            Ok(TaskPattern::Range(10..=i32::MAX))
        );
    }

    #[test]
    fn test_parse_random() {
        assert_eq!(
            TaskPattern::parse("5:random"),
            Ok(TaskPattern::Random(5, None))
        );
        assert_eq!(
            TaskPattern::parse("5:1..10"),
            Ok(TaskPattern::Random(5, Some(1..=10)))
        );
    }

    #[test]
    fn test_select_items() {
        let items: Vec<i32> = (1..=20).collect();

        // Test Exact
        let pattern = TaskPattern::Exact(5);
        assert_eq!(pattern.select_items(&items).len(), 5);

        // Test Range
        let pattern = TaskPattern::Range(5..=10);
        assert_eq!(pattern.select_items(&items), vec![6, 7, 8, 9, 10, 11]);

        // Test Random
        let pattern = TaskPattern::Random(5, None);
        assert_eq!(pattern.select_items(&items).len(), 5);

        let pattern = TaskPattern::Random(5, Some(1..=10));
        let selected = pattern.select_items(&items);
        assert_eq!(selected.len(), 5);
        assert!(selected.iter().all(|&x| (2..=11).contains(&x)));
    }

    #[test]
    fn test_serialize() {
        let pattern = TaskPattern::Exact(5);
        assert_eq!(serde_json::to_string(&pattern).unwrap(), "\"5\"");

        let pattern = TaskPattern::Range(5..=10);
        assert_eq!(serde_json::to_string(&pattern).unwrap(), "\"5..10\"");

        let pattern = TaskPattern::Random(5, Some(1..=10));
        assert_eq!(serde_json::to_string(&pattern).unwrap(), "\"5:1..10\"");

        let pattern = TaskPattern::Random(5, None);
        assert_eq!(serde_json::to_string(&pattern).unwrap(), "\"5:random\"");
    }

    #[test]
    fn test_deserialize() {
        let pattern: TaskPattern = serde_json::from_str("\"5\"").unwrap();
        assert_eq!(pattern, TaskPattern::Exact(5));

        let pattern: TaskPattern = serde_json::from_str("\"5..10\"").unwrap();
        assert_eq!(pattern, TaskPattern::Range(5..=10));

        let pattern: TaskPattern = serde_json::from_str("\"5:1..10\"").unwrap();
        assert_eq!(pattern, TaskPattern::Random(5, Some(1..=10)));

        let pattern: TaskPattern = serde_json::from_str("\"5:random\"").unwrap();
        assert_eq!(pattern, TaskPattern::Random(5, None));
    }

    #[test]
    fn test_from_usize() {
        let pattern: TaskPattern = 5.into();
        assert_eq!(pattern, TaskPattern::Exact(5));
    }

    #[test]
    fn test_from_string() {
        let pattern: TaskPattern = "5".to_string().into();
        assert_eq!(pattern, TaskPattern::Exact(5));
    }

    #[test]
    fn test_from_str() {
        let pattern: TaskPattern = "5".into();
        assert_eq!(pattern, TaskPattern::Exact(5));
    }

    #[test]
    fn test_invalid_pattern() {
        assert_eq!(
            TaskPattern::parse("invalid"),
            Err("Invalid range".to_string())
        );
    }

    #[test]
    fn test_invalid_range() {
        assert_eq!(
            TaskPattern::parse("10..5"),
            Err("Invalid range".to_string())
        );
    }

    #[test]
    fn test_invalid_random() {
        assert_eq!(
            TaskPattern::parse("5:10..5"),
            Err("Invalid range".to_string())
        );
    }

    #[test]
    fn test_invalid_random_count() {
        assert_eq!(
            TaskPattern::parse("5:random:10"),
            Err("Invalid random pattern".to_string())
        );
    }

    #[test]
    fn test_invalid_random_range() {
        assert_eq!(
            TaskPattern::parse("5:10..5"),
            Err("Invalid range".to_string())
        );
    }

    #[test]
    fn test_invalid_random_range_count() {
        assert_eq!(
            TaskPattern::parse("5:10..5:10"),
            Err("Invalid random pattern".to_string())
        );
    }
}
