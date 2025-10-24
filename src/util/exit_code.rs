// SPDX-FileCopyrightText: 2025 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::process::ExitCode as StdExitCode;

#[derive(Debug)]
pub struct ExitCode(pub i32);

impl ExitCode {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn as_std(&self) -> StdExitCode {
        StdExitCode::from(self.0 as u8)
    }
}

impl std::fmt::Display for ExitCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for ExitCode {}
