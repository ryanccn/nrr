#![warn(clippy::all, clippy::pedantic, clippy::perf)]
#![forbid(unsafe_code)]

use clap::{Parser, ValueEnum};
use owo_colors::OwoColorize;

use anyhow::Result;
use std::{env, path::PathBuf};
use tokio::fs;

mod package_json;
mod run_script;
use package_json::PackageJson;
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
}

async fn execute(
    script_name: &str,
    extra_args: Option<&Vec<String>>,
    compat_mode: Option<CompatMode>,
) -> Result<()> {
    let mut node_modules: Vec<PathBuf> = vec![];
    let mut packages: Vec<PathBuf> = vec![];

    for search_dir in env::current_dir()?.ancestors() {
        let this_nm = search_dir.join("node_modules");
        if this_nm.exists() && this_nm.is_dir() {
            node_modules.push(this_nm);
        }

        let this_pkg = search_dir.join("package.json");
        if this_pkg.exists() && this_pkg.is_file() {
            packages.push(this_pkg);
        }
    }

    let mut patched_path_vec = node_modules
        .iter()
        .map(|p| p.join(".bin"))
        .collect::<Vec<_>>();

    if let Ok(existing_path) = env::var("PATH") {
        patched_path_vec.extend(env::split_paths(&existing_path));
    }

    let patched_path = env::join_paths(patched_path_vec)?;

    let mut executed_script = false;

    for package in &packages {
        if let Ok(Ok(package_data)) = fs::read_to_string(&package)
            .await
            .map(|raw| serde_json::from_str::<PackageJson>(&raw))
        {
            if let Some(script_cmd) = package_data.scripts.get(script_name) {
                run_script(
                    package,
                    &package_data,
                    script_name,
                    script_cmd,
                    extra_args,
                    &patched_path,
                    compat_mode,
                )
                .await?;

                executed_script = true;
                break;
            }
        }
    }

    if !executed_script {
        eprintln!(
            "{}{}{}",
            "No script found with name ".red(),
            &script_name.red().bold(),
            "!".red()
        );

        eprintln!(
            "{} {}",
            "Searched:".dimmed(),
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

    Ok(())
}

#[allow(clippy::too_many_lines)]
#[tokio::main]
async fn main() -> Result<()> {
    let raw_args: Vec<String> = env::args().collect();

    let cli = if env::var_os("NRR_COMPAT_MODE").is_some_and(|v| !v.is_empty())
        && raw_args.get(1) == Some(&"run".to_owned())
    {
        let mut processed_args = raw_args.clone();
        processed_args.remove(1);

            Cli::parse_from(&processed_args)
        } else {
            Cli::parse_from(&raw_args)
        };

    execute(&cli.script, cli.extra_args.as_ref(), cli.compat).await?;

    Ok(())
}
