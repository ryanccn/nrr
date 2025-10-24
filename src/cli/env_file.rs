// SPDX-FileCopyrightText: 2025 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use dotenvy::Result;

#[derive(Clone, Debug)]
pub struct EnvFile(Vec<(String, String)>);

impl EnvFile {
    pub fn from_path(path: &str) -> Result<Self> {
        dotenvy::from_filename_iter(path)
            .and_then(|file| file.collect())
            .map(Self)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.0.iter().map(|(a, b)| (a.as_str(), b.as_str()))
    }
}
