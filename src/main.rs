mod cli;
mod package_json;
mod run;
mod suggest;
mod util;

use clap::{CommandFactory as _, Parser as _};
use clap_complete::CompleteEnv;

use std::{env, path::Path, process::ExitCode};

use crate::cli::{Cli, NrxCli};

fn main_result() -> color_eyre::Result<()> {
    color_eyre::install()?;
    util::signals::install()?;

    if env::args().next().is_some_and(|s| {
        Path::new(&s)
            .file_name()
            .is_some_and(|f| f == "nrx" || f == "nrx.exe")
    }) {
        CompleteEnv::with_factory(|| NrxCli::command().bin_name("nrx")).complete();
        NrxCli::parse().execute()?;
    } else {
        CompleteEnv::with_factory(|| Cli::command()).complete();
        Cli::parse().execute()?;
    }

    Ok(())
}

fn main() -> ExitCode {
    if let Err(err) = main_result() {
        if let Some(exit_code) = err.downcast_ref::<util::ExitCode>() {
            return exit_code.as_std();
        }

        eprintln!("Error: {err:?}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
