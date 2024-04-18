use color_eyre::eyre::Result;
use owo_colors::{OwoColorize as _, Stream};
use std::fs;
use std::path::PathBuf;

use crate::package_json::PackageJson;
use crate::run;

use super::ExecArgs;

pub fn handle(package_paths: impl Iterator<Item = PathBuf>, args: &ExecArgs) -> Result<()> {
    let mut executed_exec = false;

    for package_path in package_paths {
        if let Ok(Ok(package)) =
            fs::read(&package_path).map(|mut raw| simd_json::from_slice::<PackageJson>(&mut raw))
        {
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

        std::process::exit(1);
    }

    Ok(())
}
