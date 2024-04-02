use color_eyre::eyre::Result;
use owo_colors::{OwoColorize as _, Stream};
use std::{fs, path::PathBuf};

use crate::package_json::PackageJson;
use crate::run::run_script;
use crate::suggest::suggest;

use super::RunArgs;

pub fn handle(package_paths: impl Iterator<Item = PathBuf>, args: &RunArgs) -> Result<()> {
    let mut resolved_packages: Vec<PackageJson> = Vec::new();
    let mut executed_script = false;

    for package_path in package_paths {
        if let Ok(Ok(package)) =
            fs::read(&package_path).map(|mut raw| simd_json::from_slice::<PackageJson>(&mut raw))
        {
            if let Some(script_cmd) = package.scripts.get(&args.script) {
                run_script(&package_path, &package, &args.script, script_cmd, args)?;

                executed_script = true;
                break;
            }

            resolved_packages.push(package);
        }
    }

    if !executed_script {
        eprintln!(
            "{}  No script found with name {}.",
            "error".if_supports_color(Stream::Stderr, |text| text.red()),
            &args
                .script
                .if_supports_color(Stream::Stderr, |text| text.bold()),
        );

        for resolved_package in &resolved_packages {
            if let Some(alternative) = suggest(&args.script, resolved_package) {
                eprintln!(
                    "       {} {}{}{}",
                    "Did you mean".if_supports_color(Stream::Stderr, |text| text.dimmed()),
                    "`".if_supports_color(Stream::Stderr, |text| text.dimmed()),
                    format!("nrr {alternative}")
                        .if_supports_color(Stream::Stderr, |text| text.cyan())
                        .if_supports_color(Stream::Stderr, |text| text.dimmed()),
                    "`?".if_supports_color(Stream::Stderr, |text| text.dimmed())
                );

                break;
            }
        }

        std::process::exit(1);
    }

    Ok(())
}
