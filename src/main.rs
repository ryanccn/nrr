#![warn(clippy::all, clippy::pedantic, clippy::perf)]
#![forbid(unsafe_code)]

use clap::{Parser, ValueEnum};
use owo_colors::OwoColorize;

use anyhow::Result;
use std::{env, path::PathBuf};
use tokio::fs;

mod package_json;
mod run_script;
use package_json::{make_package_prefix, PackageJson};
use run_script::run_script;

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
    script: String,
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
    async fn execute(&self) -> Result<()> {
        let mut executed_script = false;
        let mut searched_dirs: Vec<PathBuf> = vec![];

        for search_dir in env::current_dir()?.ancestors() {
            let this_pkg = search_dir.join("package.json");

            if this_pkg.exists() && this_pkg.is_file() {
                if let Ok(Ok(package_data)) = fs::read_to_string(&this_pkg)
                    .await
                    .map(|raw| serde_json::from_str::<PackageJson>(&raw))
                {
                    if let Some(script_cmd) = package_data.scripts.get(&self.script) {
                        eprint!("{}", make_package_prefix(&package_data));

                        if self.pre_post {
                            let pre_script_name = "pre".to_owned() + &self.script;
                            if let Some(pre_script_cmd) = package_data.scripts.get(&pre_script_name)
                            {
                                run_script(
                                    &this_pkg,
                                    &package_data,
                                    &pre_script_name,
                                    pre_script_cmd,
                                    None,
                                    self.compat,
                                )
                                .await?;
                            }
                        }

                        run_script(
                            &this_pkg,
                            &package_data,
                            &self.script,
                            script_cmd,
                            self.extra_args.as_ref(),
                            self.compat,
                        )
                        .await?;

                        if self.pre_post {
                            let post_script_name = "post".to_owned() + &self.script;
                            if let Some(post_script_cmd) =
                                package_data.scripts.get(&post_script_name)
                            {
                                run_script(
                                    &this_pkg,
                                    &package_data,
                                    &post_script_name,
                                    post_script_cmd,
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

                searched_dirs.push(this_pkg);
            }
        }

        if !executed_script {
            eprintln!(
                "{}{}{}",
                "No script found with name ".red(),
                &self.script.red().bold(),
                "!".red()
            );

            eprintln!(
                "{} {}",
                "Searched:".dimmed(),
                if searched_dirs.is_empty() {
                    "<no packages found>".to_owned()
                } else {
                    searched_dirs
                        .iter()
                        .map(|p| p.to_string_lossy())
                        .collect::<Vec<_>>()
                        .join(", ")
                        .dimmed()
                        .to_string()
                }
            );
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let raw_args: Vec<String> = env::args().collect();

    let cli = Cli::parse_from(
        if env::var_os("NRR_COMPAT_MODE").is_some_and(|v| !v.is_empty())
            && raw_args.get(1) == Some(&"run".to_owned())
        {
            let mut processed_args = raw_args.clone();
            processed_args.remove(1);

            processed_args
        } else {
            raw_args
        },
    );

    cli.execute().await?;

    Ok(())
}
