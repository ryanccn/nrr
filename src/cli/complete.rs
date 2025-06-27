use clap_complete::CompletionCandidate;
use color_eyre::eyre::Result;

use std::{
    collections::{BTreeMap, BTreeSet},
    env,
    ffi::OsString,
    fs,
    path::Path,
};

use crate::package_json::PackageJson;

fn scripts_fallible() -> Result<Vec<CompletionCandidate>> {
    let current_dir = env::current_dir()?;
    let package_paths = current_dir
        .ancestors()
        .map(|p| p.join("package.json"))
        .filter(|p| p.is_file())
        .collect::<Vec<_>>();

    let mut scripts: BTreeMap<String, String> = BTreeMap::new();

    for path in package_paths.into_iter().rev() {
        if let Some(package) = PackageJson::from_path_safe(&path) {
            scripts.extend(package.scripts.into_iter());
        }
    }

    Ok(scripts
        .into_iter()
        .map(|(k, v)| CompletionCandidate::new(&k).help(Some(v.into())))
        .collect())
}

pub fn scripts() -> Vec<CompletionCandidate> {
    scripts_fallible().unwrap_or_default()
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use rustix::fs::{Access, access};
    access(path, Access::EXEC_OK).is_ok()
}

#[cfg(windows)]
fn is_executable(path: &Path) -> bool {
    winsafe::GetBinaryType(path.to_string_lossy().as_ref()).is_ok()
}

#[cfg(not(any(unix, windows)))]
fn is_executable(path: &Path) -> bool {
    true
}

fn executables_fallible() -> Result<Vec<CompletionCandidate>> {
    let current_dir = env::current_dir()?;
    let node_modules_bin_paths = current_dir
        .ancestors()
        .map(|p| p.join("node_modules").join(".bin"))
        .filter(|p| p.is_dir());

    let mut executables: BTreeSet<OsString> = BTreeSet::new();

    for path in node_modules_bin_paths {
        for entry in (fs::read_dir(&path)?).flatten() {
            if entry.path().is_file() && is_executable(&entry.path()) {
                executables.insert(entry.file_name());
            }
        }
    }

    Ok(executables
        .into_iter()
        .map(|k| CompletionCandidate::new(&k))
        .collect())
}

pub fn executables() -> Vec<CompletionCandidate> {
    executables_fallible().unwrap_or_default()
}
