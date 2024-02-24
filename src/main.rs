#![warn(clippy::all, clippy::pedantic, clippy::perf)]
#![allow(
    clippy::redundant_closure_for_method_calls,
    clippy::module_name_repetitions
)]
#![forbid(unsafe_code)]

use clap::{CommandFactory, Parser};
use color_eyre::Result;
use owo_colors::{OwoColorize, Stream};

use package_json::PackageJsonOwned;
use smartstring::alias::String;
use std::{env, fs, path::Path, sync::OnceLock};

mod package_json;
mod run_script;
mod suggest;

use crate::package_json::PackageJson;
use crate::run_script::{run_script, ScriptType};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// Minimal, blazing fast Node.js script runner
struct Cli {
    /// The name of the script
    script: Option<String>,

    /// Extra arguments to pass to the script
    #[clap(allow_hyphen_values = true)]
    extra_args: Option<Vec<String>>,

    /// Run pre- and post- scripts
    #[arg(short, long)]
    pre_post: bool,
}

fn get_level() -> &'static usize {
    static ONCE_LOCK: OnceLock<usize> = OnceLock::new();
    ONCE_LOCK.get_or_init(|| {
        std::env::var("NRR_LEVEL")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(1)
    })
}

impl Cli {
    fn run_script_full(
        &self,
        script_name: &str,
        script_cmd: &str,
        package: &Path,
        package_data: &PackageJson<'_, '_, '_>,
    ) -> Result<()> {
        eprint!(
            "{}",
            package_data.make_prefix(match get_level() {
                1 => None,
                _ => Some(script_name),
            })
        );

        if self.pre_post {
            let pre_script_name = String::from("pre") + script_name;

            if let Some(pre_script_cmd) = package_data.scripts.get(&pre_script_name) {
                run_script(
                    package,
                    package_data,
                    &pre_script_name,
                    pre_script_cmd,
                    ScriptType::Pre,
                    None,
                )?;
            }
        }

        run_script(
            package,
            package_data,
            script_name,
            script_cmd,
            ScriptType::Normal,
            self.extra_args.as_ref(),
        )?;

        if self.pre_post {
            let post_script_name = String::from("post") + script_name;

            if let Some(post_script_cmd) = package_data.scripts.get(&post_script_name) {
                run_script(
                    package,
                    package_data,
                    &post_script_name,
                    post_script_cmd,
                    ScriptType::Post,
                    None,
                )?;
            }
        }

        Ok(())
    }

    fn execute(&self) -> Result<()> {
        let current_dir = env::current_dir()?;
        let package_paths = current_dir
            .ancestors()
            .map(|p| p.join("package.json"))
            .filter(|p| p.is_file());

        let mut resolved_packages = Vec::<PackageJsonOwned>::new();

        if let Some(script_name) = &self.script {
            let mut executed_script = false;

            for package_path in package_paths {
                if let Ok(raw) = fs::read_to_string(&package_path) {
                    if let Ok(package) = serde_json::from_str::<PackageJson>(&raw) {
                        if let Some(script_cmd) = package.scripts.get(script_name) {
                            self.run_script_full(script_name, script_cmd, &package_path, &package)?;

                            executed_script = true;
                            break;
                        }

                        resolved_packages.push(package.to_owned());
                    }
                }
            }

            if !executed_script {
                eprintln!(
                    "{}  No script found with name {}.",
                    "error".if_supports_color(Stream::Stderr, |text| text.red()),
                    script_name.if_supports_color(Stream::Stderr, |text| text.bold()),
                );

                for resolved_package in &resolved_packages {
                    if let Some(alternative) = suggest::suggest(script_name, resolved_package) {
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
        } else {
            let mut found_package = false;

            for package in package_paths {
                let raw = fs::read_to_string(package)?;
                if let Ok(package_data) = serde_json::from_str::<PackageJson>(&raw) {
                    print!("{}", package_data.make_prefix(None));

                    found_package = true;

                    for (script_name, script_content) in &package_data.scripts {
                        println!(
                            "{}",
                            script_name.if_supports_color(Stream::Stdout, |text| text.cyan())
                        );
                        println!("  {script_content}");
                    }

                    println!();
                }
            }

            if !found_package {
                Cli::command().print_help()?;
            }
        };

        Ok(())
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    #[allow(clippy::disallowed_types)]
    let mut raw_args: Vec<std::string::String> = env::args().collect();

    if env::var_os("NRR_COMPAT_MODE").is_some_and(|v| !v.is_empty())
        && raw_args
            .get(1)
            .is_some_and(|v| v == "run" || v == "run-script")
    {
        raw_args.remove(1);
    }

    let cli = Cli::parse_from(raw_args);
    cli.execute()?;

    Ok(())
}
