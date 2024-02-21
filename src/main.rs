use clap::{CommandFactory, Parser, ValueEnum};
use owo_colors::{OwoColorize, Stream};

use color_eyre::Result;
use std::sync::OnceLock;
use std::{env, path::Path};
use tokio::fs;

mod package_json;
mod run_script;
mod serde_util;

use crate::package_json::PackageJson;
use crate::run_script::{run_script, ScriptType};

#[derive(ValueEnum, PartialEq, Eq, Debug, Clone, Copy)]
pub enum CompatMode {
    Npm,
    Yarn,
    Pnpm,
    Bun,
}

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
    async fn run_script_full(
        &self,
        script_name: &str,
        script_cmd: &str,
        package: &Path,
        package_data: &PackageJson<'_>,
    ) -> Result<()> {
        eprint!(
            "{}",
            package_data.make_prefix(match get_level() {
                1 => None,
                _ => Some(script_name),
            })
        );

        if self.pre_post {
            let pre_script_name = "pre".to_owned() + script_name;

            if let Some(pre_script_cmd) = package_data.scripts.get(&pre_script_name) {
                run_script(
                    package,
                    package_data,
                    &pre_script_name,
                    pre_script_cmd,
                    ScriptType::Pre,
                    None,
                )
                .await?;
            }
        }

        run_script(
            package,
            package_data,
            script_name,
            script_cmd,
            ScriptType::Normal,
            self.extra_args.as_ref(),
        )
        .await?;

        if self.pre_post {
            let post_script_name = "post".to_owned() + script_name;

            if let Some(post_script_cmd) = package_data.scripts.get(&post_script_name) {
                run_script(
                    package,
                    package_data,
                    &post_script_name,
                    post_script_cmd,
                    ScriptType::Post,
                    None,
                )
                .await?;
            }
        }

        Ok(())
    }

    async fn execute(&self) -> Result<()> {
        let current_dir = env::current_dir()?;
        let packages = current_dir
            .ancestors()
            .map(|p| p.join("package.json"))
            .filter(|p| p.is_file());

        if let Some(script_name) = &self.script {
            let mut executed_script = false;

            for package in packages.clone() {
                let raw = fs::read_to_string(&package).await?;
                if let Ok(package_data) = serde_json::from_str::<PackageJson>(&raw) {
                    if let Some(script_cmd) = package_data.scripts.get(script_name) {
                        self.run_script_full(script_name, script_cmd, &package, &package_data)
                            .await?;

                        executed_script = true;
                        break;
                    }
                }
            }

            if !executed_script {
                eprintln!(
                    "{}{}{}",
                    "No script found with name "
                        .if_supports_color(Stream::Stderr, |text| text.red()),
                    script_name
                        .if_supports_color(Stream::Stderr, |text| text.red())
                        .if_supports_color(Stream::Stderr, |text| text.bold()),
                    "!".if_supports_color(Stream::Stderr, |text| text.red())
                );

                let packages = packages.collect::<Vec<_>>();

                eprintln!(
                    "{} {}",
                    "Searched:".if_supports_color(Stream::Stderr, |text| text.dimmed()),
                    if packages.is_empty() {
                        "<no packages found>".to_owned()
                    } else {
                        packages
                            .iter()
                            .map(|p| p.to_string_lossy())
                            .collect::<Vec<_>>()
                            .join(", ")
                            .dimmed()
                            .to_string()
                    }
                );
            }
        } else {
            let mut found_package = false;

            for package in packages {
                let raw = fs::read_to_string(package).await?;
                if let Ok(package_data) = serde_json::from_str::<PackageJson>(&raw) {
                    eprint!("{}", package_data.make_prefix(None));

                    found_package = true;

                    let mut all_scripts = package_data.scripts.iter().collect::<Vec<_>>();
                    all_scripts.sort_by_key(|s| s.0);

                    for (script_name, script_content) in &all_scripts {
                        println!(
                            "{}",
                            script_name.if_supports_color(Stream::Stdout, |text| text.cyan())
                        );
                        println!("  {script_content}");
                    }
                }
            }

            if !found_package {
                Cli::command().print_help()?;
            }
        };

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let mut raw_args: Vec<String> = env::args().collect();

    if env::var_os("NRR_COMPAT_MODE").is_some_and(|v| !v.is_empty())
        && raw_args
            .get(1)
            .is_some_and(|v| v == "run" || v == "run-script")
    {
        raw_args.remove(1);
    }

    let cli = Cli::parse_from(raw_args);
    cli.execute().await?;

    Ok(())
}
