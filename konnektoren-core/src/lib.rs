pub mod challenges;
pub mod player_profile;
pub mod session;

/// experience points
pub type Xp = i32;

pub mod prelude {
    pub use crate::challenges::ChallengeType;
    pub use crate::challenges::MultipleChoice;
    pub use crate::challenges::MultipleChoiceOption;
    pub use crate::challenges::Question;
    pub use crate::player_profile::PlayerProfile;
    pub use crate::session::Session;
    pub use crate::Xp;
}
