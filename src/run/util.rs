// SPDX-FileCopyrightText: 2025 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use color_eyre::eyre::Result;
use std::{
    env,
    ffi::OsString,
    path::{Path, PathBuf},
    process::Command,
};

#[must_use]
pub fn make_patched_paths(package_path: &Path) -> Vec<PathBuf> {
    let mut patched_path = package_path
        .ancestors()
        .map(|p| p.join("node_modules").join(".bin"))
        .filter(|p| p.is_dir())
        .collect::<Vec<_>>();

    if let Ok(existing_path) = env::var("PATH") {
        patched_path.extend(env::split_paths(&existing_path));
    }

    patched_path
}

pub fn make_patched_path(package_path: &Path) -> Result<OsString> {
    Ok(env::join_paths(make_patched_paths(package_path))?)
}

#[cfg(unix)]
#[inline]
pub fn make_shell_cmd() -> Command {
    let mut cmd = Command::new("/bin/sh");
    cmd.arg("-c");
    cmd
}

#[cfg(windows)]
#[inline]
pub fn make_shell_cmd() -> Command {
    let mut cmd = Command::new("cmd.exe");
    cmd.args(["/d", "/s", "/c"]);
    cmd
}
