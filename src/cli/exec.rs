// SPDX-FileCopyrightText: 2025 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use color_eyre::eyre::Result;
use owo_colors::{OwoColorize as _, Stream};
use std::path::PathBuf;

use crate::{package_json::PackageJson, run, util::ExitCode};

use super::ExecArgs;

pub fn handle(package_paths: impl Iterator<Item = PathBuf>, args: &ExecArgs) -> Result<()> {
    let mut executed_exec = false;

    for package_path in package_paths {
        if let Some(package) = PackageJson::from_path_safe(&package_path) {
            run::exec(&package_path, &package, args)?;
            executed_exec = true;
            break;
        }
    }

    if !executed_exec {
        eprintln!(
            "{}  No packages found!",
            "error".if_supports_color(Stream::Stderr, |text| text.red()),
        );

        return Err(ExitCode(1).into());
    }

    Ok(())
}
