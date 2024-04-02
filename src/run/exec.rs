#[cfg(unix)]
use nix::{
    sys::signal::{kill, Signal},
    unistd::Pid,
};

use std::{env, path::Path, process::Command};

use color_eyre::Result;
use owo_colors::{OwoColorize as _, Stream};

#[cfg(unix)]
use ctrlc::set_handler;

use crate::{cli::ExecArgs, package_json::PackageJson, run::util};

pub fn run_exec(package_path: &Path, package_data: &PackageJson, args: &ExecArgs) -> Result<()> {
    let package_folder = package_path.parent().unwrap();

    if !args.silent {
        let cmd_prefix = "$".repeat(*crate::get_level());

        eprintln!(
            "{} {} {}",
            cmd_prefix
                .if_supports_color(Stream::Stderr, |text| text.cyan())
                .if_supports_color(Stream::Stderr, |text| text.dimmed()),
            args.bin
                .if_supports_color(Stream::Stderr, |text| text.dimmed()),
            args.extra_args
                .join(" ")
                .if_supports_color(Stream::Stderr, |text| text.dimmed())
        );
    }

    let mut subproc = Command::new(&args.bin);
    subproc.current_dir(package_folder).args(&args.extra_args);

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

    let mut child = subproc.spawn()?;

    #[allow(clippy::cast_possible_wrap)]
    #[cfg(unix)]
    let pid = Pid::from_raw(child.id() as i32);

    #[cfg(unix)]
    set_handler(move || {
        kill(pid, Signal::SIGINT).ok();
    })?;

    let status = child.wait()?;

    if !status.success() {
        let code = status.code().unwrap_or(1);

        if !args.silent {
            eprintln!(
                "{}  Exited with status {}!",
                "error".if_supports_color(Stream::Stderr, |text| text.red()),
                util::itoa(code).if_supports_color(Stream::Stderr, |text| text.bold()),
            );
        }

        std::process::exit(code);
    }

    Ok(())
}
