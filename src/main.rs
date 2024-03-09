pub mod cli;
pub mod package_json;
pub mod run;
pub mod suggest;

use std::env;

pub use cli::get_level;

use crate::cli::{Cli, NrxCli};
use clap::Parser as _;

use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    if env::current_exe().is_ok_and(|exe| exe.file_name().is_some_and(|f| f == "nrx")) {
        let cli = NrxCli::parse();
        cli.execute()?;
    } else {
        let cli = Cli::parse();
        cli.execute()?;
    }

    Ok(())
}
