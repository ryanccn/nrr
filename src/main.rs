#![warn(clippy::all, clippy::pedantic, clippy::perf)]
#![forbid(unsafe_code)]

use clap::{CommandFactory, Parser, ValueEnum};
use owo_colors::OwoColorize;

use color_eyre::Result;
use std::{env, path::Path};
use tokio::fs;

mod package_json;
mod run_script;
use package_json::{make_package_prefix, PackageJson};
use run_script::run_script;

use crate::run_script::ScriptType;

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
    extra_args: Option<Vec<String>>,

    /// Apply additional behaviors to emulate a specific package manager
    #[arg(short, long)]
    compat: Option<CompatMode>,

    /// Run pre- and post- scripts
    #[arg(short, long)]
    pre_post: bool,
}

impl Cli {
    #[allow(clippy::too_many_lines)]
    async fn execute(&self) -> Result<()> {
        let packages = env::current_dir()?
            .ancestors()
            .map(|p| p.join("package.json"))
            .filter(|p| p.is_file())
            .collect::<Vec<_>>();

        if let Some(self_script) = &self.script {
            let mut executed_script = false;
            let mut searched_pkgs: Vec<&Path> = vec![];

            for package in &packages {
                if let Ok(Ok(package_data)) = fs::read_to_string(package)
                    .await
                    .map(|raw| serde_json::from_str::<PackageJson>(&raw))
                {
                    if let Some(script_cmd) = package_data.scripts.get(self_script) {
                        eprint!("{}", make_package_prefix(&package_data));

                        if self.pre_post {
                            let pre_script_name = "pre".to_owned() + self_script;

                            if let Some(pre_script_cmd) = package_data.scripts.get(&pre_script_name)
                            {
                                run_script(
                                    package,
                                    &package_data,
                                    &pre_script_name,
                                    pre_script_cmd,
                                    ScriptType::Pre,
                                    None,
                                    self.compat,
                                )
                                .await?;
                            }
                        }

                        run_script(
                            package,
                            &package_data,
                            self_script,
                            script_cmd,
                            ScriptType::Normal,
                            self.extra_args.as_ref(),
                            self.compat,
                        )
                        .await?;

                        if self.pre_post {
                            let post_script_name = "post".to_owned() + self_script;

                            if let Some(post_script_cmd) =
                                package_data.scripts.get(&post_script_name)
                            {
                                run_script(
                                    package,
                                    &package_data,
                                    &post_script_name,
                                    post_script_cmd,
                                    ScriptType::Post,
                                    None,
                                    self.compat,
                                )
                                .await?;
                            }
                        }

                        executed_script = true;
                        break;
                    }
                }

                searched_pkgs.push(package);
            }

            if !executed_script {
                eprintln!(
                    "{}{}{}",
                    "No script found with name ".red(),
                    self_script.red().bold(),
                    "!".red()
                );

                eprintln!(
                    "{} {}",
                    "Searched:".dimmed(),
                    if searched_pkgs.is_empty() {
                        "<no packages found>".to_owned()
                    } else {
                        searched_pkgs
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
                if let Ok(Ok(package_data)) = fs::read_to_string(package)
                    .await
                    .map(|raw| serde_json::from_str::<PackageJson>(&raw))
                {
                    eprint!("{}", make_package_prefix(&package_data));

                    found_package = true;

                    let mut all_scripts = package_data.scripts.iter().collect::<Vec<_>>();
                    all_scripts.sort_by_key(|s| s.0);

                    for (script_name, script_content) in &all_scripts {
                        println!("{}", script_name.cyan());
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
