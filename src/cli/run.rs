use color_eyre::eyre::Result;
use owo_colors::{OwoColorize as _, Stream};
use std::path::PathBuf;
use std::{fs, path::Path};

use crate::package_json::{PackageJson, PackageJsonOwned};
use crate::run::{run_script, ScriptType};
use crate::suggest::suggest;

use super::RunArgs;

fn run_script_full(
    package: &Path,
    package_data: &PackageJson,
    script_name: &str,
    script_cmd: &str,
    extra_args: &[String],
    no_pre_post: bool,
    silent: bool,
) -> Result<()> {
    if !silent {
        eprint!(
            "{}",
            package_data.make_prefix(
                match crate::get_level() {
                    1 => None,
                    _ => Some(script_name),
                },
                Stream::Stderr
            )
        );
    }

    if !no_pre_post {
        let pre_script_name = String::from("pre") + script_name;

        if let Some(pre_script_cmd) = package_data.scripts.get(&pre_script_name) {
            run_script(
                ScriptType::Pre,
                package,
                package_data,
                &pre_script_name,
                pre_script_cmd,
                &[],
                silent,
            )?;
        }
    }

    run_script(
        ScriptType::Normal,
        package,
        package_data,
        script_name,
        script_cmd,
        extra_args,
        silent,
    )?;

    if !no_pre_post {
        let post_script_name = String::from("post") + script_name;

        if let Some(post_script_cmd) = package_data.scripts.get(&post_script_name) {
            run_script(
                ScriptType::Post,
                package,
                package_data,
                &post_script_name,
                post_script_cmd,
                &[],
                silent,
            )?;
        }
    }

    Ok(())
}

pub fn handle(package_paths: impl Iterator<Item = PathBuf>, args: &RunArgs) -> Result<()> {
    let mut resolved_packages = Vec::<PackageJsonOwned>::new();
    let mut executed_script = false;

    for package_path in package_paths {
        if let Ok(raw) = fs::read_to_string(&package_path) {
            if let Ok(package) = serde_json::from_str::<PackageJson>(&raw) {
                if let Some(script_cmd) = package.scripts.get(&args.script) {
                    run_script_full(
                        &package_path,
                        &package,
                        &args.script,
                        script_cmd,
                        &args.extra_args,
                        args.no_pre_post,
                        args.silent,
                    )?;

                    executed_script = true;
                    break;
                }

                resolved_packages.push(package.to_owned(package_path));
            }
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
