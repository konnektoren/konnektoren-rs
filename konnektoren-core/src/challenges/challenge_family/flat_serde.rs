//! Challenge families are saved without any marker of the branch: the value is written as the
//! included type would write it, and loading picks the branch by the variant name.
//!
//! This works for tagged formats too (YAML `!name`, JSON `{"name": ...}`, named MessagePack
//! maps), which `#[serde(untagged)]` cannot read back.

use super::{ChallengeFamily, ChallengeFamilyResult};
use crate::challenges::SerializedNames;
use serde::de::value::StringDeserializer;
use serde::de::{self, DeserializeSeed, Deserializer, EnumAccess, IntoDeserializer, Visitor};
use serde::{Deserialize, Serialize, Serializer};
use std::fmt;
use std::marker::PhantomData;

/// A deserializer for one included enum whose variant name was already read.
struct Replay<Va> {
    name: String,
    variant: Va,
}

struct ReplayEnum<Va> {
    name: String,
    variant: Va,
}

impl<'de, Va: de::VariantAccess<'de>> EnumAccess<'de> for ReplayEnum<Va> {
    type Error = Va::Error;
    type Variant = Va;

    fn variant_seed<S: DeserializeSeed<'de>>(self, seed: S) -> Result<(S::Value, Va), Va::Error> {
        let name: StringDeserializer<Va::Error> = self.name.into_deserializer();
        Ok((seed.deserialize(name)?, self.variant))
    }
}

impl<'de, Va: de::VariantAccess<'de>> Deserializer<'de> for Replay<Va> {
    type Error = Va::Error;

    fn deserialize_enum<Vis: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: Vis,
    ) -> Result<Vis::Value, Va::Error> {
        visitor.visit_enum(ReplayEnum {
            name: self.name,
            variant: self.variant,
        })
    }

    fn deserialize_any<Vis: Visitor<'de>>(self, _visitor: Vis) -> Result<Vis::Value, Va::Error> {
        Err(de::Error::custom(
            "challenge family branches must be enums with named variants",
        ))
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf
        option unit unit_struct newtype_struct seq tuple tuple_struct map struct identifier
        ignored_any
    }
}

struct FamilyVisitor<A, B, T> {
    marker: PhantomData<(A, B, T)>,
}

/// Which branch a saved value belongs to, and the value for it.
enum Branch<A, B> {
    First(A),
    Second(B),
}

fn pick_branch<'de, A, B, E>(data: E) -> Result<Branch<A, B>, E::Error>
where
    A: Deserialize<'de> + SerializedNames,
    B: Deserialize<'de> + SerializedNames,
    E: EnumAccess<'de>,
{
    let (name, variant) = data.variant::<String>()?;
    let first = A::serialized_names().contains(&name.as_str());
    let second = B::serialized_names().contains(&name.as_str());
    if first && second {
        return Err(de::Error::custom(format!(
            "ambiguous variant `{name}` in challenge family"
        )));
    }
    if first {
        A::deserialize(Replay { name, variant }).map(Branch::First)
    } else if second {
        B::deserialize(Replay { name, variant }).map(Branch::Second)
    } else {
        Err(de::Error::custom(format!("unknown variant `{name}`")))
    }
}

impl<'de, A, B> Visitor<'de> for FamilyVisitor<A, B, ChallengeFamily<A, B>>
where
    A: Deserialize<'de> + SerializedNames,
    B: Deserialize<'de> + SerializedNames,
{
    type Value = ChallengeFamily<A, B>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a challenge of one of the included types")
    }

    fn visit_enum<E: EnumAccess<'de>>(self, data: E) -> Result<Self::Value, E::Error> {
        Ok(match pick_branch::<A, B, E>(data)? {
            Branch::First(a) => ChallengeFamily::First(a),
            Branch::Second(b) => ChallengeFamily::Second(b),
        })
    }
}

impl<'de, A, B> Visitor<'de> for FamilyVisitor<A, B, ChallengeFamilyResult<A, B>>
where
    A: Deserialize<'de> + SerializedNames,
    B: Deserialize<'de> + SerializedNames,
{
    type Value = ChallengeFamilyResult<A, B>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a result of one of the included types")
    }

    fn visit_enum<E: EnumAccess<'de>>(self, data: E) -> Result<Self::Value, E::Error> {
        Ok(match pick_branch::<A, B, E>(data)? {
            Branch::First(a) => ChallengeFamilyResult::First(a),
            Branch::Second(b) => ChallengeFamilyResult::Second(b),
        })
    }
}

fn ensure_unique_names<A: SerializedNames, B: SerializedNames, E: serde::ser::Error>()
-> Result<(), E> {
    let mut names = std::collections::HashSet::new();
    for name in A::serialized_names()
        .into_iter()
        .chain(B::serialized_names())
    {
        if !names.insert(name) {
            return Err(E::custom(format!(
                "ambiguous variant `{name}` in challenge family"
            )));
        }
    }
    Ok(())
}

impl<A: Serialize + SerializedNames, B: Serialize + SerializedNames> Serialize
    for ChallengeFamily<A, B>
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        ensure_unique_names::<A, B, S::Error>()?;
        match self {
            Self::First(a) => a.serialize(serializer),
            Self::Second(b) => b.serialize(serializer),
        }
    }
}

impl<'de, A, B> Deserialize<'de> for ChallengeFamily<A, B>
where
    A: Deserialize<'de> + SerializedNames,
    B: Deserialize<'de> + SerializedNames,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_enum(
            "ChallengeFamily",
            &[],
            FamilyVisitor::<A, B, Self> {
                marker: PhantomData,
            },
        )
    }
}

impl<A: Serialize + SerializedNames, B: Serialize + SerializedNames> Serialize
    for ChallengeFamilyResult<A, B>
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        ensure_unique_names::<A, B, S::Error>()?;
        match self {
            Self::First(a) => a.serialize(serializer),
            Self::Second(b) => b.serialize(serializer),
        }
    }
}

impl<'de, A, B> Deserialize<'de> for ChallengeFamilyResult<A, B>
where
    A: Deserialize<'de> + SerializedNames,
    B: Deserialize<'de> + SerializedNames,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_enum(
            "ChallengeFamilyResult",
            &[],
            FamilyVisitor::<A, B, Self> {
                marker: PhantomData,
            },
        )
    }
}

impl<A: SerializedNames, B: SerializedNames> SerializedNames for ChallengeFamily<A, B> {
    fn serialized_names() -> Vec<&'static str> {
        let mut names = A::serialized_names();
        names.extend(B::serialized_names());
        names
    }
}

impl<A: SerializedNames, B: SerializedNames> SerializedNames for ChallengeFamilyResult<A, B> {
    fn serialized_names() -> Vec<&'static str> {
        let mut names = A::serialized_names();
        names.extend(B::serialized_names());
        names
    }
}

impl<A: SerializedNames, B: SerializedNames> ChallengeFamily<A, B> {
    /// Saved names used by both branches. Loading would give them to the first branch, so
    /// a family with duplicates should not be used; check this in a test or at startup.
    pub fn duplicate_names() -> Vec<&'static str> {
        let second = B::serialized_names();
        A::serialized_names()
            .into_iter()
            .filter(|name| second.contains(name))
            .collect()
    }
}
