use owo_colors::{OwoColorize, Stream};
use std::borrow::Cow;

use crate::serde_util;
use serde::Deserialize;
use smartstring::alias::String;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson<'a> {
    #[serde(borrow, deserialize_with = "serde_util::de_opt_cow_str")]
    pub name: Option<Cow<'a, str>>,
    #[serde(borrow, deserialize_with = "serde_util::de_opt_cow_str")]
    pub version: Option<Cow<'a, str>>,

    #[serde(borrow, default, deserialize_with = "serde_util::de_hashmap_cow_str")]
    pub scripts: serde_util::AIndexMap<String, Cow<'a, str>>,
}

impl PackageJson<'_> {
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
