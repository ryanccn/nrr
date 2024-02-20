use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Deserializer};

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
) -> Result<HashMap<String, Cow<'de, str>>, D::Error>
where
    D: Deserializer<'de>,
{
    HashMap::<String, WrappedCowStr<'de>>::deserialize(deserializer).map(|val| {
        val.into_iter()
            .map(|(k, v)| (k, v.0))
            .collect::<HashMap<_, _>>()
    })
}
