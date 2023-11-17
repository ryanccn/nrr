use nix::{
    sys::signal::{kill, Signal},
    unistd::Pid,
};
use which::which;

use anyhow::Result;
use owo_colors::OwoColorize;
use std::{
    env,
    ffi::OsString,
    path::{Path, PathBuf},
};
use tokio::{fs, process::Command, signal::ctrl_c};

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
    let mut nm_bin_folders: Vec<PathBuf> = vec![];

    for search_dir in package_path.ancestors() {
        let this_nm = search_dir.join("node_modules");
        if this_nm.exists() && this_nm.is_dir() {
            nm_bin_folders.push(this_nm.join(".bin"));
        }
    }

    if let Ok(existing_path) = env::var("PATH") {
        nm_bin_folders.extend(env::split_paths(&existing_path));
    }

    Ok(env::join_paths(nm_bin_folders)?)
}

#[allow(clippy::too_many_lines)]
pub async fn run_script(
    package_path: &Path,
    package_data: &PackageJson,
    script_name: &str,
    script_cmd: &str,
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

    let cmd_prefix = if script_name.starts_with("pre") {
        "pre$"
    } else if script_name.starts_with("post") {
        "post$"
    } else {
        "$"
    };

    eprintln!("{} {}", cmd_prefix.cyan().dimmed(), full_cmd.dimmed());

    let mut subproc = make_shell_cmd()?;
    subproc.current_dir(package_folder).arg(&full_cmd);

    let self_exe = env::current_exe()?;
    let patched_path = make_patched_path(package_path)?;

    subproc
        .env("PATH", patched_path)
        .env("NRR_COMPAT_MODE", "1")
        .env("npm_execpath", &self_exe)
        .env("npm_lifecycle_event", script_name);

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
        if let Ok(Ok(package_data_raw)) = fs::read_to_string(package_path)
            .await
            .map(|txt| serde_json::from_str::<serde_json::Value>(&txt))
        {
            subproc.envs(serialize_package_json_env(&package_data_raw));
        }
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
    let pid = child.id().map(|v| Pid::from_raw(v as i32));

    let task = tokio::task::spawn(async move {
        if ctrl_c().await.is_ok() {
            if let Some(pid) = pid {
                kill(pid, Signal::SIGINT).ok();
            }
        }
    });

    if let Ok(status) = child.wait().await {
        task.abort();
        if !status.success() {
            std::process::exit(status.code().unwrap_or(1));
        }
    }

    Ok(())
}
