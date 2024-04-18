use owo_colors::{OwoColorize as _, Stream};
use serde::Deserialize;

use indexmap::IndexMap;

pub type AIndexMap<K, V> = IndexMap<K, V, ahash::RandomState>;

#[derive(Deserialize, Clone, Debug)]
pub struct PackageJson {
    pub name: Option<String>,
    pub version: Option<String>,

    #[serde(default)]
    pub scripts: AIndexMap<String, String>,
}

impl PackageJson {
    #[must_use]
    pub fn make_prefix(&self, script: Option<&str>, stream: Stream) -> String {
        let mut prefix = String::new();

        if let Some(name) = &self.name {
            prefix.push_str(
                &name
                    .if_supports_color(stream, |text| text.magenta())
                    .to_string(),
            );

            if let Some(version) = &self.version {
                let mut version_str = String::new();
                version_str.push('@');
                version_str.push_str(version);
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
                        .if_supports_color(Stream::Stderr, |text| text.cyan())
                        .to_string(),
                );
            }

            prefix.push('\n');
        }

        prefix
    }
}
