use super::Yaml;
use konnektoren_asset_loader::AssetFormat;
use konnektoren_core::challenges::{ChallengeFamily, ChallengeType};
use serde::de::DeserializeOwned;
use std::{collections::HashSet, error::Error, marker::PhantomData};

/// A YAML challenge parser and the type names it accepts, without leading `!`.
pub trait TaggedChallengeFormat: AssetFormat + Clone {
    fn challenge_types(&self) -> Vec<&'static str>;
}

impl TaggedChallengeFormat for Yaml<ChallengeType> {
    fn challenge_types(&self) -> Vec<&'static str> {
        ChallengeType::type_names().collect()
    }
}

/// YAML format for an application-defined type and its accepted YAML tags.
pub struct TaggedYaml<T> {
    types: Vec<&'static str>,
    marker: PhantomData<fn() -> T>,
}

impl<T> TaggedYaml<T> {
    pub fn new(types: impl IntoIterator<Item = &'static str>) -> Self {
        Self {
            types: types.into_iter().collect(),
            marker: PhantomData,
        }
    }
}

impl<T> Clone for TaggedYaml<T> {
    fn clone(&self) -> Self {
        Self::new(self.types.iter().copied())
    }
}

impl<T: DeserializeOwned> AssetFormat for TaggedYaml<T> {
    type Asset = T;
    type Error = serde_yaml::Error;
    fn cache_key(&self) -> Option<String> {
        Some(self.types.join("\0"))
    }
    fn parse(&self, bytes: &[u8]) -> Result<T, Self::Error> {
        serde_yaml::from_slice(bytes)
    }
}

impl<T: DeserializeOwned> TaggedChallengeFormat for TaggedYaml<T> {
    fn challenge_types(&self) -> Vec<&'static str> {
        self.types.clone()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FamilyFormatError {
    #[error("duplicate or invalid challenge type: {0}")]
    DuplicateOrInvalidType(String),
    #[error("invalid challenge YAML: {0}")]
    InvalidYaml(#[from] serde_yaml::Error),
    #[error("challenge YAML must have a type tag")]
    MissingType,
    #[error("unsupported challenge type: {0}")]
    UnknownType(String),
    #[error("failed to parse challenge type {challenge_type}: {source}")]
    Parse {
        challenge_type: String,
        #[source]
        source: Box<dyn Error + Send + Sync>,
    },
}

/// Composes two challenge parsers while keeping authored YAML tags flat.
#[derive(Clone)]
pub struct ChallengeFamilyFormat<A, B> {
    first: A,
    second: B,
    first_types: Vec<&'static str>,
    second_types: Vec<&'static str>,
}

impl<A: TaggedChallengeFormat, B: TaggedChallengeFormat> ChallengeFamilyFormat<A, B> {
    pub fn new(first: A, second: B) -> Result<Self, FamilyFormatError> {
        let first_types = first.challenge_types();
        let second_types = second.challenge_types();
        let mut names = HashSet::new();
        for name in first_types.iter().chain(&second_types) {
            if name.is_empty() || name.starts_with('!') || !names.insert(*name) {
                return Err(FamilyFormatError::DuplicateOrInvalidType(
                    (*name).to_owned(),
                ));
            }
        }
        Ok(Self {
            first,
            second,
            first_types,
            second_types,
        })
    }
}

impl<A: TaggedChallengeFormat, B: TaggedChallengeFormat> AssetFormat
    for ChallengeFamilyFormat<A, B>
{
    type Asset = ChallengeFamily<A::Asset, B::Asset>;
    type Error = FamilyFormatError;

    fn extensions(&self) -> &[&str] {
        &["yml", "yaml"]
    }

    fn cache_key(&self) -> Option<String> {
        let first = self.first.cache_key()?;
        let second = self.second.cache_key()?;
        serde_json::to_string(&(&self.first_types, &self.second_types, first, second)).ok()
    }

    fn parse(&self, bytes: &[u8]) -> Result<Self::Asset, Self::Error> {
        let value: serde_yaml::Value = serde_yaml::from_slice(bytes)?;
        let serde_yaml::Value::Tagged(tagged) = value else {
            return Err(FamilyFormatError::MissingType);
        };
        let tag = tagged.tag.to_string();
        let name = tag.strip_prefix('!').unwrap_or(&tag);
        if self.first_types.contains(&name) {
            self.first
                .parse(bytes)
                .map(ChallengeFamily::First)
                .map_err(|source| FamilyFormatError::Parse {
                    challenge_type: name.to_owned(),
                    source: Box::new(source),
                })
        } else if self.second_types.contains(&name) {
            self.second
                .parse(bytes)
                .map(ChallengeFamily::Second)
                .map_err(|source| FamilyFormatError::Parse {
                    challenge_type: name.to_owned(),
                    source: Box::new(source),
                })
        } else {
            Err(FamilyFormatError::UnknownType(name.to_owned()))
        }
    }
}

impl<A: TaggedChallengeFormat, B: TaggedChallengeFormat> TaggedChallengeFormat
    for ChallengeFamilyFormat<A, B>
{
    fn challenge_types(&self) -> Vec<&'static str> {
        self.first_types
            .iter()
            .chain(&self.second_types)
            .copied()
            .collect()
    }
}
