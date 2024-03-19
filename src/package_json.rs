use owo_colors::{OwoColorize, Stream};
use serde::Deserialize;
use serde_with::{serde_as, BorrowCow};

use indexmap::IndexMap;
use std::{borrow::Cow, path::PathBuf};

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

#[derive(Clone, Debug)]
pub struct PackageJsonOwned {
    pub path: PathBuf,
    pub name: Option<String>,
    pub version: Option<String>,
    pub scripts: AIndexMap<String, String>,
}

fn make_prefix_internal(
    name: Option<impl Into<String>>,
    version: Option<impl Into<String>>,
    script: Option<impl Into<String>>,
    stream: Stream,
) -> String {
    let mut prefix = String::new();

    if let Some(name) = name {
        prefix.push_str(
            &name
                .into()
                .if_supports_color(stream, |text| text.magenta())
                .to_string(),
        );

        if let Some(version) = version {
            let mut version_str = String::new();
            version_str.push('@');
            version_str.push_str(&version.into());
            prefix.push_str(
                &version_str
                    .if_supports_color(stream, |text| text.magenta())
                    .if_supports_color(stream, |text| text.dimmed())
                    .to_string(),
            );
        }

        if let Some(script) = script {
            prefix.push(' ');
            prefix.push_str(
                &script
                    .into()
                    .if_supports_color(Stream::Stderr, |text| text.cyan())
                    .to_string(),
            );
        }

        prefix.push('\n');
    }

    prefix
}

impl PackageJson<'_, '_, '_> {
    #[must_use]
    pub fn to_owned(&self, path: PathBuf) -> PackageJsonOwned {
        PackageJsonOwned {
            path,
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

    #[allow(dead_code)]
    #[must_use]
    pub fn make_prefix(&self, script_name: Option<&str>, stream: Stream) -> String {
        make_prefix_internal(
            self.name.as_ref().map(|v| v.as_ref()),
            self.version.as_ref().map(|v| v.as_ref()),
            script_name,
            stream,
        )
    }
}

impl PackageJsonOwned {
    #[allow(dead_code)]
    #[must_use]
    pub fn make_prefix(&self, script_name: Option<&str>, stream: Stream) -> String {
        make_prefix_internal(
            self.name.as_ref(),
            self.version.as_ref(),
            script_name,
            stream,
        )
    }
}
