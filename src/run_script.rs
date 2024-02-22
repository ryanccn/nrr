#[cfg(unix)]
use nix::{
    sys::signal::{kill, Signal},
    unistd::Pid,
};

use smartstring::alias::String;
use std::{env, ffi::OsString, path::Path, process::Command};

use color_eyre::Result;
use owo_colors::{OwoColorize, Stream};

#[cfg(unix)]
use ctrlc::set_handler;

use crate::package_json::PackageJson;

#[cfg(unix)]
#[allow(clippy::unnecessary_wraps)]
#[inline]
fn make_shell_cmd() -> Result<Command> {
    let mut cmd = Command::new("sh");
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

#[inline]
fn make_patched_path(package_path: &Path) -> Result<OsString> {
    let mut patched_path = package_path
        .ancestors()
        .map(|p| p.join("node_modules").join(".bin"))
        .filter(|p| p.is_dir())
        .collect::<Vec<_>>();

    if let Ok(existing_path) = env::var("PATH") {
        patched_path.extend(env::split_paths(&existing_path));
    }

    Ok(env::join_paths(patched_path)?)
}

#[derive(Copy, Clone, PartialEq)]
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
    package_path: &Path,
    package_data: &PackageJson<'_>,
    script_name: &str,
    script_cmd: &str,
    script_type: ScriptType,
    extra_args: Option<&Vec<String>>,
) -> Result<()> {
    let package_folder = package_path.parent().unwrap();

    let mut full_cmd = script_cmd.to_owned();

    if let Some(extra_args) = extra_args {
        if !extra_args.is_empty() {
            full_cmd.push(' ');
            extra_args
                .iter()
                .map(|f| shlex::try_quote(f))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .for_each(|arg| full_cmd.push_str(arg.as_ref()));
        }
    }

    let cmd_prefix = script_type.prefix() + &"$".repeat(*crate::get_level());

    eprintln!(
        "{} {}",
        cmd_prefix
            .if_supports_color(Stream::Stderr, |text| text.cyan())
            .if_supports_color(Stream::Stderr, |text| text.dimmed()),
        full_cmd.if_supports_color(Stream::Stderr, |text| text.dimmed())
    );

    let mut subproc = make_shell_cmd()?;
    subproc.current_dir(package_folder).arg(&full_cmd);

    let self_exe = env::current_exe()?;
    let patched_path = make_patched_path(package_path)?;

    subproc.env("PATH", patched_path);

    subproc
        .env("NRR_COMPAT_MODE", "1")
        .env("NRR_LEVEL", format!("{}", crate::get_level() + 1));

    subproc
        .env("npm_execpath", &self_exe)
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

        eprintln!(
            "{} {}{}",
            "Exited with status".if_supports_color(Stream::Stderr, |text| text.red()),
            code.to_string()
                .if_supports_color(Stream::Stderr, |text| text.red())
                .if_supports_color(Stream::Stderr, |text| text.bold()),
            "!".if_supports_color(Stream::Stderr, |text| text.red())
        );

        std::process::exit(code);
    }

    Ok(())
}
