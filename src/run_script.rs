#[cfg(unix)]
use nix::{
    sys::signal::{kill, Signal},
    unistd::Pid,
};
use which::which;

use std::{env, ffi::OsString, path::Path};
use tokio::process::Command;

use color_eyre::Result;
use owo_colors::OwoColorize;

#[cfg(unix)]
use tokio::signal::ctrl_c;

use crate::{
    package_json::{serialize_package_json_env, PackageJson},
    CompatMode,
};

#[cfg(unix)]
#[allow(clippy::unnecessary_wraps)]
fn make_shell_cmd() -> Result<Command> {
    let mut cmd = Command::new("sh");
    cmd.arg("-c");
    Ok(cmd)
}

#[cfg(windows)]
#[allow(clippy::unnecessary_wraps)]
fn make_shell_cmd() -> Result<Command> {
    let mut cmd = Command::new(env::var("ComSpec")?);
    cmd.args(["/d", "/s", "/c"]);
    Ok(cmd)
}

fn make_patched_path(package_path: &Path) -> Result<OsString> {
    let mut nm_bin_folders = package_path
        .ancestors()
        .map(|p| p.join("node_modules").join(".bin"))
        .filter(|p| p.is_dir())
        .collect::<Vec<_>>();

    if let Ok(existing_path) = env::var("PATH") {
        nm_bin_folders.extend(env::split_paths(&existing_path));
    }

    Ok(env::join_paths(nm_bin_folders)?)
}

#[derive(Copy, Clone, PartialEq)]
pub enum ScriptType {
    Pre,
    Post,
    Normal,
}

#[allow(clippy::too_many_lines)]
pub async fn run_script(
    package_path: &Path,
    package_data: &PackageJson,
    script_name: &str,
    script_cmd: &str,
    script_type: ScriptType,
    extra_args: Option<&Vec<String>>,
    compat_mode: Option<CompatMode>,
) -> Result<()> {
    let package_folder = package_path.parent().unwrap();

    let mut full_cmd = script_cmd.to_owned();

    if let Some(extra_args) = extra_args {
        if !extra_args.is_empty() {
            full_cmd = full_cmd
                + " "
                + &extra_args
                    .iter()
                    .map(|f| shell_escape::escape(f.into()).into_owned())
                    .collect::<Vec<String>>()
                    .join(" ");
        }
    }

    let cmd_prefix = match script_type {
        ScriptType::Normal => String::new(),
        ScriptType::Pre => "pre".to_owned(),
        ScriptType::Post => "post".to_owned(),
    } + &"$".repeat(crate::get_level());

    eprintln!("{} {}", cmd_prefix.cyan().dimmed(), full_cmd.dimmed());

    let mut subproc = make_shell_cmd()?;
    subproc.current_dir(package_folder).arg(&full_cmd);

    let self_exe = env::current_exe()?;
    let patched_path = make_patched_path(package_path)?;

    subproc
        .env("PATH", patched_path)
        .env("npm_execpath", &self_exe)
        .env("npm_lifecycle_event", script_name);

    subproc
        .env("NRR_COMPAT_MODE", "1")
        .env("NRR_LEVEL", format!("{}", crate::get_level() + 1));

    if let Ok(node_path) = which(match compat_mode {
        Some(CompatMode::Bun) => "bun",
        _ => "node",
    }) {
        subproc
            .env("NODE", &node_path)
            .env("npm_node_execpath", &node_path);
    }

    if compat_mode == Some(CompatMode::Npm) {
        subproc.env("COLOR", "0");
    } else if compat_mode == Some(CompatMode::Yarn) {
        subproc.env("YARN_WRAP_OUTPUT", "false");
    } else if compat_mode == Some(CompatMode::Pnpm) {
        subproc.env("PNPM_SCRIPT_SRC_DIR", package_folder);
    }

    if compat_mode != Some(CompatMode::Bun) {
        subproc.env("npm_lifecycle_script", &full_cmd);
        if let Ok(cwd) = env::current_dir() {
            subproc.env("INIT_CWD", &cwd);
        }
    }

    if let Some(CompatMode::Npm | CompatMode::Bun) = compat_mode {
        subproc.env("npm_package_json", package_path);
    }

    if let Some(CompatMode::Npm | CompatMode::Pnpm) = compat_mode {
        subproc.env("npm_command", "run-script");
    }

    if let Some(CompatMode::Yarn | CompatMode::Pnpm) = compat_mode {
        let package_value = serde_json::to_value(package_data)?;
        subproc.envs(serialize_package_json_env(&package_value));
    } else {
        if let Some(p_name) = &package_data.name {
            subproc.env("npm_package_name", p_name);
        }
        if let Some(p_version) = &package_data.version {
            subproc.env("npm_package_version", p_version);
        }
    }

    let mut child = subproc.spawn()?;

    #[allow(clippy::cast_possible_wrap)]
    #[cfg(unix)]
    let pid = child.id().map(|v| Pid::from_raw(v as i32));

    #[cfg(unix)]
    let task = tokio::task::spawn(async move {
        if ctrl_c().await.is_ok() {
            if let Some(pid) = pid {
                kill(pid, Signal::SIGINT).ok();
            }
        }
    });

    let status = child.wait().await?;

    #[cfg(unix)]
    task.abort();

    if !status.success() {
        let code = status.code().unwrap_or(1);

        eprintln!(
            "{} {}{}",
            "Exited with status".red(),
            code.to_string().red().bold(),
            "!".red()
        );
        std::process::exit(code);
    }

    Ok(())
}
