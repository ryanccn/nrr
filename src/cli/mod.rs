use clap::{Args, CommandFactory, Parser, Subcommand, ValueHint};
use clap_complete::ArgValueCandidates;

use color_eyre::Result;
use owo_colors::{OwoColorize as _, Stream};
use std::env;

use self::env_file::EnvFile;

mod complete;
mod env_file;
mod exec;
mod list;
mod run;

#[derive(Parser, Clone, Debug)]
#[clap(version, about, long_about = None, args_conflicts_with_subcommands = true)]
pub struct Cli {
    #[clap(flatten)]
    root_args: RootArgs,

    #[clap(subcommand)]
    subcommand: Option<Subcommands>,
}

#[derive(Parser, Clone, Debug)]
#[clap(version, about, long_about = None, args_conflicts_with_subcommands = true)]
pub struct NrxCli {
    #[clap(flatten)]
    args: ExecArgs,
}

#[derive(Subcommand, Clone, Debug)]
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

#[derive(Args, Clone, Debug)]
pub struct RootArgs {
    /// The name of the script to run
    #[clap(add = ArgValueCandidates::new(complete::scripts))]
    pub script: Option<String>,

    #[clap(flatten)]
    pub options: SharedRunOptions,
}

#[derive(Args, Clone, Debug)]
pub struct RunArgs {
    /// The name of the script to run
    #[clap(add = ArgValueCandidates::new(complete::scripts))]
    pub script: String,

    #[clap(flatten)]
    pub options: SharedRunOptions,
}

#[derive(Args, Clone, Debug)]
pub struct SharedRunOptions {
    /// Arguments to pass to the script
    #[clap(trailing_var_arg = true, allow_hyphen_values = true)]
    pub args: Vec<String>,

    /// Don't run pre- and post- scripts
    #[arg(short, long, env = "NRR_NO_PRE_POST")]
    pub no_pre_post: bool,

    /// Disable printing package and command information
    #[clap(short, long, env = "NRR_SILENT")]
    pub silent: bool,

    /// An environment file to read environment variables from
    #[clap(short, long, env = "NRR_ENV_FILE", value_hint = ValueHint::FilePath, value_parser = EnvFile::from_path)]
    pub env_file: Option<EnvFile>,
}

#[derive(Args, Clone, Debug)]
pub struct ExecArgs {
    /// The name of the executable to run
    #[clap(add = ArgValueCandidates::new(complete::executables))]
    pub executable: String,

    /// Arguments to pass to the executable
    #[clap(trailing_var_arg = true, allow_hyphen_values = true)]
    pub args: Vec<String>,

    /// An environment file to read environment variables from
    #[clap(short, long, env = "NRR_ENV_FILE", value_hint = ValueHint::FilePath, value_parser = EnvFile::from_path)]
    pub env_file: Option<EnvFile>,
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

                    std::process::exit(1);
                }
            }

            None => {
                if let Some(script_name) = &self.root_args.script {
                    run::handle(
                        package_paths,
                        &RunArgs {
                            script: script_name.to_owned(),
                            options: self.root_args.options.clone(),
                        },
                    )?;
                } else {
                    let found_package = list::handle(package_paths)?;
                    if !found_package {
                        Self::command().print_help()?;
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
