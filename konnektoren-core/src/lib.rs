pub mod challenges;
pub mod commands;
pub mod controller;
pub mod events;
pub mod game;
pub mod i18n;
pub mod persistence;
pub mod player_profile;
pub mod session;

#[cfg(feature = "achievements")]
pub mod achievements;

#[cfg(feature = "certificates")]
pub mod certificates;

#[cfg(feature = "js")]
pub mod konnektoren_js;

#[cfg(feature = "marketplace")]
pub mod marketplace;

/// experience points
pub type Xp = u32;

/// This is a prelude module that re-exports the most important types and traits.
pub mod prelude {
    pub use crate::challenges::Challenge;
    pub use crate::challenges::ChallengeConfig;
    pub use crate::challenges::ChallengeFactory;
    pub use crate::challenges::ChallengeInput;
    pub use crate::challenges::ChallengeResult;
    pub use crate::challenges::ChallengeType;
    pub use crate::challenges::MultipleChoice;
    pub use crate::challenges::MultipleChoiceOption;
    pub use crate::challenges::Performance;
    pub use crate::challenges::Question;
    pub use crate::game::Game;
    pub use crate::game::GamePath;
    pub use crate::i18n::Language;
    pub use crate::player_profile::PlayerProfile;
    pub use crate::session::Session;
    pub use crate::Xp;

    #[cfg(feature = "achievements")]
    pub use crate::achievements::*;

    #[cfg(feature = "js")]
    pub use crate::konnektoren_js::*;

    #[cfg(feature = "marketplace")]
    pub use crate::marketplace::*;
}

#[cfg(feature = "js")]
#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod tests {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
}
