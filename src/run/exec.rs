use std::{env, path::Path, process::Command};

use color_eyre::Result;
use owo_colors::{OwoColorize as _, Stream};

#[cfg(unix)]
use std::os::unix::process::CommandExt as _;

use crate::{cli::ExecArgs, package_json::PackageJson, run::util};

pub fn run_exec(package_path: &Path, package_data: &PackageJson, args: &ExecArgs) -> Result<()> {
    let package_folder = package_path.parent().unwrap();

    let escaped_command = args
        .command
        .iter()
        .map(|f| shlex::try_quote(f))
        .collect::<Result<Vec<_>, _>>()?
        .join(" ");

    if !args.silent {
        let cmd_prefix = "$".repeat(*crate::get_level());

        eprintln!(
            "{} {}",
            cmd_prefix
                .if_supports_color(Stream::Stderr, |text| text.cyan())
                .if_supports_color(Stream::Stderr, |text| text.dimmed()),
            escaped_command.if_supports_color(Stream::Stderr, |text| text.dimmed()),
        );
    }

    let mut command_iter = args.command.iter();
    let mut subproc = Command::new(command_iter.next().unwrap());
    subproc.current_dir(package_folder).args(command_iter);

    if let Some(env_file) = &args.env_file {
        subproc.envs(env_file.iter());
    }

    subproc
        .env("PATH", util::make_patched_path(package_path)?)
        .env("__NRR_LEVEL", util::itoa(crate::get_level() + 1));

    subproc
        .env("npm_execpath", env::current_exe()?)
        .env("npm_package_json", package_path);

    if let Some(name) = &package_data.name {
        subproc.env("npm_package_name", name);
    }
    if let Some(version) = &package_data.version {
        subproc.env("npm_package_version", version);
    }

    #[cfg(unix)]
    {
        Err::<(), _>(subproc.exec())?;
        Ok(())
    }

    #[cfg(windows)]
    {
        let _ = ctrlc::set_handler(|| {});
        let status = subproc.status()?;
        std::process::exit(status.code().unwrap_or(1));
    }
}
