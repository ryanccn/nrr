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

use crate::{cli::RunArgs, package_json::PackageJson, run::util};

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

fn run_single_script(
    script_type: ScriptType,
    package_path: &Path,
    package_data: &PackageJson,
    script_name: &str,
    script_cmd: &str,
    args: &RunArgs,
) -> Result<()> {
    let package_folder = package_path.parent().unwrap();

    let mut full_cmd = script_cmd.to_owned();

    if script_type == ScriptType::Normal {
        args.extra_args
            .iter()
            .map(|f| shlex::try_quote(f))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .for_each(|arg| {
                full_cmd.push(' ');
                full_cmd.push_str(arg.as_ref());
            });
    }

    if !args.silent {
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

    if let Some(env_file) = &args.env_file {
        subproc.envs(env_file.iter());
    }

    subproc
        .env("PATH", util::make_patched_path(package_path)?)
        .env("__NRR_LEVEL", util::itoa(crate::get_level() + 1));

    subproc
        .env("npm_execpath", env::current_exe()?)
        .env("npm_lifecycle_event", script_name)
        .env("npm_lifecycle_script", &full_cmd);

    subproc.env("npm_package_json", package_path);

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

pub fn run_script(
    package_path: &Path,
    package_data: &PackageJson,
    script_name: &str,
    script_cmd: &str,
    args: &RunArgs,
) -> Result<()> {
    if !args.silent {
        eprint!(
            "{}",
            package_data.make_prefix(
                match crate::get_level() {
                    1 => None,
                    _ => Some(script_name),
                },
                Stream::Stderr
            )
        );
    }

    if !args.no_pre_post {
        let pre_script_name = String::from("pre") + script_name;

        if let Some(pre_script_cmd) = package_data.scripts.get(&pre_script_name) {
            run_single_script(
                ScriptType::Pre,
                package_path,
                package_data,
                &pre_script_name,
                pre_script_cmd,
                args,
            )?;
        }
    }

    run_single_script(
        ScriptType::Normal,
        package_path,
        package_data,
        script_name,
        script_cmd,
        args,
    )?;

    if !args.no_pre_post {
        let post_script_name = String::from("post") + script_name;

        if let Some(post_script_cmd) = package_data.scripts.get(&post_script_name) {
            run_single_script(
                ScriptType::Post,
                package_path,
                package_data,
                &post_script_name,
                post_script_cmd,
                args,
            )?;
        }
    }

    Ok(())
}
