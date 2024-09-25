//! This module contains all the components that are used in the app.

mod certificates;
pub mod challenge;
pub mod challenge_config;
pub mod challenge_info;
pub mod challenge_rating;
pub mod challenge_review;
pub mod game_path;
mod map;
#[cfg(feature = "music")]
pub mod music;
pub mod profile;
pub mod progress_bar;
pub mod select_language;
mod select_level;
mod settings;
pub mod translate;

#[cfg(feature = "marketplace")]
pub mod marketplace;

pub use challenge::*;
pub use challenge_config::ChallengeConfigComponent;
pub use challenge_info::ChallengeInfoComponent;
pub use challenge_rating::ChallengeRatingComponent;
pub use challenge_review::ChallengeReviewComponent;
pub use game_path::GamePathComponent;
pub use map::*;
pub use progress_bar::ProgressBar;
pub use select_language::SelectLanguage;
pub use select_level::SelectLevelComp;
pub use translate::TranslateComponent;

#[cfg(feature = "certificates")]
pub use certificates::*;

#[cfg(feature = "storage")]
pub use profile::ProfileConfigComponent;
#[cfg(feature = "storage")]
pub use profile::ProfilePointsComponent;

#[cfg(feature = "music")]
pub use music::MusicComponent;

#[cfg(feature = "marketplace")]
pub use marketplace::*;

pub use settings::*;
