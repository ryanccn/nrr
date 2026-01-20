// SPDX-FileCopyrightText: 2025 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use color_eyre::Result;
use std::{env, path::Path};

use crate::{
    cli::ExecArgs,
    package_json::PackageJson,
    run::util::make_shell_cmd,
    util::{ExitCode, NRR_LEVEL, itoa, signals},
};

use super::util::make_patched_path;

pub fn exec(package_path: &Path, package_data: &PackageJson, args: &ExecArgs) -> Result<()> {
    let mut shell_script = vec![args.executable.as_str()];
    shell_script.extend(args.args.iter().map(|i| i.as_str()));

    let mut command = make_shell_cmd(
        #[cfg(unix)]
        &shell_script
            .into_iter()
            .map(|f| shlex::try_quote(f))
            .collect::<Result<Vec<_>, _>>()?
            .join(" "),
        #[cfg(windows)]
        &shell_script.join(" "),
    );

    if let Some(env_file) = &args.env_file {
        command.envs(env_file.iter());
    }

    command
        .env("PATH", make_patched_path(package_path)?)
        .env("__NRR_LEVEL", itoa(*NRR_LEVEL + 1))
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
