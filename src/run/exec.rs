use color_eyre::Result;
use std::{env, path::Path, process::Command};

use crate::{
    cli::ExecArgs,
    package_json::PackageJson,
    util::{itoa, signals, ExitCode, NRR_LEVEL},
};

use super::util::make_patched_path;

pub fn exec(package_path: &Path, package_data: &PackageJson, args: &ExecArgs) -> Result<()> {
    let package_folder = package_path.parent().unwrap();

    let mut command = Command::new(&args.executable);
    command.current_dir(package_folder).args(&args.args);

    if let Some(env_file) = &args.env_file {
        command.envs(env_file.iter());
    }

    command
        .env("PATH", make_patched_path(package_path)?)
        .env("__NRR_LEVEL", itoa(*NRR_LEVEL + 1));

    command
        .env("npm_execpath", env::current_exe()?)
        .env("npm_package_json", package_path);

    if let Some(name) = &package_data.name {
        command.env("npm_package_name", name);
    }
    if let Some(version) = &package_data.version {
        command.env("npm_package_version", version);
    }

    signals::ignore();
    let mut child = command.spawn()?;
    let status = child.wait()?;
    signals::restore();

    Err(ExitCode(status.code().unwrap_or(1)).into())
}
