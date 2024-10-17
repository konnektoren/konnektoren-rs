//! This module contains all the challenges that can be solved by the user.
pub mod challenge;
pub mod challenge_config;
pub mod challenge_factory;
pub mod challenge_history;
pub mod challenge_input;
pub mod challenge_result;
pub mod challenge_stats;
pub mod challenge_type;
pub mod challenge_variant;
pub mod contextual_choice;
pub mod custom;
pub mod informative;
pub mod multiple_choice;
#[cfg(feature = "js")]
pub mod package;
pub mod performance;
pub mod performance_record;
pub mod review;
pub mod solvable;
pub mod sort_table;
pub mod task_pattern;

pub use challenge::Challenge;
pub use challenge_config::ChallengeConfig;
pub use challenge_factory::ChallengeFactory;
pub use challenge_history::ChallengeHistory;
pub use challenge_input::ChallengeInput;
pub use challenge_result::ChallengeResult;
pub use challenge_stats::ChallengeStats;
pub use challenge_type::ChallengeType;
pub use challenge_variant::ChallengeVariant;
pub use contextual_choice::*;
pub use custom::*;
pub use informative::{Informative, InformativeText};
pub use multiple_choice::*;
#[cfg(feature = "js")]
pub use package::*;
pub use performance::Performance;
pub use performance_record::PerformanceRecord;
pub use review::Review;
pub use solvable::Solvable;
pub use sort_table::{SortTable, SortTableRow};
