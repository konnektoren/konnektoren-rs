use crate::challenges::{ChallengeConfig, Custom};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct PackageMetadata {
    pub config: ChallengeConfig,
    pub custom: Custom,
}
