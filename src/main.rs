mod cli;
mod package_json;
mod run;
mod suggest;
mod util;

use clap::Parser as _;
use color_eyre::eyre::Result;
use std::{env, path::PathBuf};

use crate::cli::{Cli, NrxCli};

fn main() -> Result<()> {
    color_eyre::install()?;

    if env::args()
        .next()
        .and_then(|s| s.parse::<PathBuf>().ok())
        .is_some_and(|exe| {
            exe.file_name()
                .is_some_and(|f| f == "nrx" || f == "nrx.exe")
        })
    {
        let cli = NrxCli::parse();
        cli.execute()?;
    } else {
        let cli = Cli::parse();
        cli.execute()?;
    }

    Ok(())
}
