use clap::{Args, CommandFactory, Parser, Subcommand};
use color_eyre::Result;
use owo_colors::{OwoColorize, Stream};

use std::{env, sync::OnceLock};

mod exec;
mod list;
mod run;

/// Minimal, blazing fast npm scripts runner.
#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(flatten)]
    root_args: RootArgs,

    #[clap(subcommand)]
    subcommand: Option<Subcommands>,
}

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct NrxCli {
    #[clap(flatten)]
    args: ExecArgs,
}

#[derive(Subcommand, Clone)]
#[command(args_conflicts_with_subcommands = true)]
enum Subcommands {
    /// Run a script
    #[clap(visible_alias = "run-script")]
    Run(RunArgs),

    /// Execute a command
    #[clap(visible_alias = "x")]
    Exec(ExecArgs),

    /// List available scripts
    #[clap(visible_alias = "ls")]
    List,
}

#[derive(Args, Clone)]
struct RootArgs {
    /// The name of the script
    script: Option<String>,

    /// Extra arguments to pass to the script
    #[clap(allow_hyphen_values = true)]
    extra_args: Vec<String>,

    /// Don't run pre- and post- scripts
    #[arg(short, long)]
    no_pre_post: bool,
}

#[derive(Args, Clone)]
struct RunArgs {
    /// The name of the script
    script: String,

    /// Extra arguments to pass to the script
    #[clap(allow_hyphen_values = true)]
    extra_args: Vec<String>,

    /// Don't run pre- and post- scripts
    #[arg(short, long)]
    no_pre_post: bool,
}

#[derive(Args, Clone)]
struct ExecArgs {
    /// The name of the command
    bin: String,

    /// Extra arguments to pass to the command
    #[clap(allow_hyphen_values = true)]
    extra_args: Vec<String>,
}

pub fn get_level() -> &'static usize {
    static ONCE_LOCK: OnceLock<usize> = OnceLock::new();
    ONCE_LOCK.get_or_init(|| {
        std::env::var("NRR_LEVEL")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(1)
    })
}

impl Cli {
    pub fn execute(&self) -> Result<()> {
        let current_dir = env::current_dir()?;
        let package_paths = current_dir
            .ancestors()
            .map(|p| p.join("package.json"))
            .filter(|p| p.is_file());

        match &self.subcommand {
            Some(Subcommands::Run(args)) => {
                run::handle(package_paths, args)?;
            }

            Some(Subcommands::Exec(args)) => {
                exec::handle(package_paths, args)?;
            }

            Some(Subcommands::List) => {
                let found_package = list::handle(package_paths)?;
                if !found_package {
                    eprintln!(
                        "{}  No packages found!",
                        "error".if_supports_color(Stream::Stderr, |text| text.red()),
                    );
                }
            }

            None => {
                if let Some(script_name) = &self.root_args.script {
                    run::handle(
                        package_paths,
                        &RunArgs {
                            script: script_name.to_owned(),
                            extra_args: self.root_args.extra_args.clone(),
                            no_pre_post: self.root_args.no_pre_post,
                        },
                    )?;
                } else {
                    let found_package = list::handle(package_paths)?;
                    if !found_package {
                        Cli::command().print_help()?;
                    }
                }
            }
        };

        Ok(())
    }
}

impl NrxCli {
    pub fn execute(&self) -> Result<()> {
        let current_dir = env::current_dir()?;
        let package_paths = current_dir
            .ancestors()
            .map(|p| p.join("package.json"))
            .filter(|p| p.is_file());

        exec::handle(package_paths, &self.args)?;

        Ok(())
    }
}
