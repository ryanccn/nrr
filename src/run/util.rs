use color_eyre::eyre::Result;
use std::{
    env,
    ffi::OsString,
    path::{Path, PathBuf},
    process::Command,
};

#[must_use]
pub fn itoa(input: impl itoa::Integer) -> String {
    itoa::Buffer::new().format(input).to_owned()
}

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

#[must_use]
pub fn has_exec(package_path: &Path, bin: &str) -> bool {
    make_patched_paths(package_path)
        .into_iter()
        .map(|p| p.join(bin))
        .any(|p| p.is_file())
}

#[cfg(unix)]
#[allow(clippy::unnecessary_wraps)]
#[inline]
pub fn make_shell_cmd() -> Result<Command> {
    let mut cmd = Command::new("/bin/sh");
    cmd.arg("-c");
    Ok(cmd)
}

#[cfg(windows)]
#[allow(clippy::unnecessary_wraps)]
#[inline]
pub fn make_shell_cmd() -> Result<Command> {
    let mut cmd = Command::new(env::var("ComSpec")?);
    cmd.args(["/d", "/s", "/c"]);
    Ok(cmd)
}
