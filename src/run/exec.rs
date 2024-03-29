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

use crate::{package_json::PackageJson, run::util};

pub fn run_exec(
    package_path: &Path,
    package_data: &PackageJson,
    bin: &str,
    args: &[String],
    silent: bool,
) -> Result<()> {
    let package_folder = package_path.parent().unwrap();

    if !silent {
        let cmd_prefix = "$".repeat(*crate::get_level());

        eprintln!(
            "{} {} {}",
            cmd_prefix
                .if_supports_color(Stream::Stderr, |text| text.cyan())
                .if_supports_color(Stream::Stderr, |text| text.dimmed()),
            bin.if_supports_color(Stream::Stderr, |text| text.dimmed()),
            args.join(" ")
                .if_supports_color(Stream::Stderr, |text| text.dimmed())
        );
    }

    let mut subproc = Command::new(bin);
    subproc.current_dir(package_folder).args(args);

    subproc
        .env("PATH", util::make_patched_path(package_path)?)
        .env("__NRR_LEVEL", util::itoa(crate::get_level() + 1));

    subproc
        .env("npm_execpath", env::current_exe()?)
        .env("npm_package_json", package_path);

    if let Some(name) = &package_data.name {
        subproc.env("npm_package_name", name.as_ref());
    }
    if let Some(version) = &package_data.version {
        subproc.env("npm_package_version", version.as_ref());
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

        if !silent {
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
