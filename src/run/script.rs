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

#[cfg(unix)]
#[allow(clippy::unnecessary_wraps)]
#[inline]
fn make_shell_cmd() -> Result<Command> {
    let mut cmd = Command::new("/bin/sh");
    cmd.arg("-c");
    Ok(cmd)
}

#[cfg(windows)]
#[allow(clippy::unnecessary_wraps)]
#[inline]
fn make_shell_cmd() -> Result<Command> {
    let mut cmd = Command::new(env::var("ComSpec")?);
    cmd.args(["/d", "/s", "/c"]);
    Ok(cmd)
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ScriptType {
    Pre,
    Post,
    Normal,
}

impl ScriptType {
    #[must_use]
    pub fn prefix(self) -> String {
        match self {
            ScriptType::Normal => String::new(),
            ScriptType::Pre => "pre".into(),
            ScriptType::Post => "post".into(),
        }
    }
}

pub fn run_script(
    script_type: ScriptType,
    package_path: &Path,
    package_data: &PackageJson,
    script_name: &str,
    script_cmd: &str,
    extra_args: &[String],
    silent: bool,
) -> Result<()> {
    let package_folder = package_path.parent().unwrap();

    let mut full_cmd = script_cmd.to_owned();

    extra_args
        .iter()
        .map(|f| shlex::try_quote(f))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .for_each(|arg| {
            full_cmd.push(' ');
            full_cmd.push_str(arg.as_ref());
        });

    if !silent {
        let cmd_prefix = script_type.prefix() + &"$".repeat(*crate::get_level());

        eprintln!(
            "{} {}",
            cmd_prefix
                .if_supports_color(Stream::Stderr, |text| text.cyan())
                .if_supports_color(Stream::Stderr, |text| text.dimmed()),
            full_cmd.if_supports_color(Stream::Stderr, |text| text.dimmed())
        );
    }

    let mut subproc = make_shell_cmd()?;
    subproc.current_dir(package_folder).arg(&full_cmd);

    subproc
        .env("PATH", util::make_patched_path(package_path)?)
        .env("__NRR_LEVEL", util::itoa(crate::get_level() + 1));

    subproc
        .env("npm_execpath", env::current_exe()?)
        .env("npm_lifecycle_event", script_name)
        .env("npm_lifecycle_script", &full_cmd);

    subproc.env("npm_package_json", package_path);

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
