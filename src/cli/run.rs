use color_eyre::eyre::Result;
use owo_colors::{OwoColorize as _, Stream};
use std::path::PathBuf;

use crate::{package_json::PackageJson, run, suggest::suggest, util::ExitCode};

use super::RunArgs;

pub fn handle(package_paths: impl Iterator<Item = PathBuf>, args: &RunArgs) -> Result<()> {
    let mut resolved_packages: Vec<PackageJson> = Vec::new();
    let mut executed_script = false;

    for package_path in package_paths {
        if let Some(package) = PackageJson::from_path_safe(&package_path) {
            if let Some(script_cmd) = package.scripts.get(&args.script) {
                run::script(
                    &package_path,
                    &package,
                    &args.script,
                    script_cmd,
                    &args.options,
                )?;

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

        return Err(ExitCode(1).into());
    }

    Ok(())
}
