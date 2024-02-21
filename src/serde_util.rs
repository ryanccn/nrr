use smartstring::alias::String;
use std::{borrow::Cow, hash::BuildHasherDefault};

use ahash::AHasher;
use indexmap::IndexMap;
use serde::{Deserialize, Deserializer};

pub type AIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<AHasher>>;

#[derive(Deserialize, Clone, Debug)]
#[serde(transparent)]
pub struct WrappedCowStr<'a>(#[serde(borrow)] Cow<'a, str>);

pub fn de_opt_cow_str<'de, D>(deserializer: D) -> Result<Option<Cow<'de, str>>, D::Error>
where
    D: Deserializer<'de>,
{
    <Option<WrappedCowStr<'de>>>::deserialize(deserializer).map(|val| val.map(|inner| inner.0))
}

pub fn de_hashmap_cow_str<'de, D>(
    deserializer: D,
) -> Result<AIndexMap<String, Cow<'de, str>>, D::Error>
where
    D: Deserializer<'de>,
{
    AIndexMap::<String, WrappedCowStr<'de>>::deserialize(deserializer).map(|val| {
        val.into_iter()
            .map(|(k, v)| (k, v.0))
            .collect::<AIndexMap<_, _>>()
    })
}
