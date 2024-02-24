use owo_colors::{OwoColorize, Stream};
use serde::Deserialize;
use serde_with::{serde_as, BorrowCow};

use indexmap::IndexMap;
use smartstring::alias::String;
use std::borrow::Cow;

pub type AIndexMap<K, V> = IndexMap<K, V, ahash::RandomState>;

#[serde_as]
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson<'a, 'b, 'c> {
    #[serde_as(as = "Option<BorrowCow>")]
    pub name: Option<Cow<'a, str>>,
    #[serde_as(as = "Option<BorrowCow>")]
    pub version: Option<Cow<'b, str>>,

    #[serde(default)]
    #[serde_as(as = "AIndexMap<_, BorrowCow>")]
    pub scripts: AIndexMap<String, Cow<'c, str>>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PackageJsonOwned {
    pub name: Option<String>,
    pub version: Option<String>,

    #[serde(default)]
    pub scripts: AIndexMap<String, String>,
}

impl PackageJson<'_, '_, '_> {
    #[must_use]
    pub fn to_owned(&self) -> PackageJsonOwned {
        PackageJsonOwned {
            name: self.name.clone().map(|v| v.into()),
            version: self.version.clone().map(|v| v.into()),
            scripts: self
                .scripts
                .clone()
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }

    #[must_use]
    pub fn make_prefix(&self, script_name: Option<&str>) -> String {
        let mut prefix = String::new();

        if let Some(name) = &self.name {
            prefix.push_str(
                &name
                    .if_supports_color(Stream::Stderr, |text| text.magenta())
                    .to_string(),
            );

            if let Some(version) = &self.version {
                let mut version_str = String::new();
                version_str.push('@');
                version_str.push_str(version);
                prefix.push_str(
                    &version_str
                        .if_supports_color(Stream::Stderr, |text| text.magenta())
                        .if_supports_color(Stream::Stderr, |text| text.dimmed())
                        .to_string(),
                );
            }

            if let Some(script_name) = script_name {
                prefix.push(' ');
                prefix.push_str(
                    &script_name
                        .if_supports_color(Stream::Stderr, |text| text.cyan())
                        .to_string(),
                );
            }

            prefix.push('\n');
        }

        prefix
    }
}
