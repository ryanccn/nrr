use std::{fs, path::Path};

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

#[derive(Debug)]
pub enum PackageJsonFromPathError {
    FileError(std::io::Error),
    ParseError(simd_json::Error),
}

impl PackageJsonFromPathError {
    pub fn do_warn(&self, package_path: &Path) {
        match self {
            Self::ParseError(err) => {
                eprintln!(
                    "{}   {} could not be parsed: {}",
                    "warn".if_supports_color(Stream::Stderr, |text| text.yellow()),
                    &package_path
                        .to_string_lossy()
                        .if_supports_color(Stream::Stderr, |text| text.bold()),
                    &err,
                );
            }

            Self::FileError(err) => {
                eprintln!(
                    "{}   {} could not be read: {}",
                    "warn".if_supports_color(Stream::Stderr, |text| text.yellow()),
                    &package_path
                        .to_string_lossy()
                        .if_supports_color(Stream::Stderr, |text| text.bold()),
                    &err,
                );
            }
        }
    }
}

impl PackageJson {
    pub fn from_path(path: &Path) -> Result<Self, PackageJsonFromPathError> {
        use PackageJsonFromPathError::{FileError, ParseError};
        let mut raw = fs::read(path).map_err(FileError)?;
        simd_json::from_slice(&mut raw).map_err(ParseError)
    }

    pub fn from_path_safe(path: &Path) -> Option<Self> {
        match Self::from_path(path) {
            Ok(package) => Some(package),
            Err(error) => {
                error.do_warn(path);
                None
            }
        }
    }

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
