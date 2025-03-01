mod cli;
mod package_json;
mod run;
mod suggest;
mod util;

use clap::{CommandFactory as _, Parser as _};
use clap_complete::CompleteEnv;

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
        CompleteEnv::with_factory(|| NrxCli::command().bin_name("nrx")).complete();
        NrxCli::parse().execute()?;
    } else {
        CompleteEnv::with_factory(|| Cli::command()).complete();
        Cli::parse().execute()?;
    }

    Ok(())
}
