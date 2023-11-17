use nix::{
    sys::signal::{kill, Signal},
    unistd::Pid,
};
use which::which;

use anyhow::Result;
use owo_colors::OwoColorize;
use std::{env, ffi::OsStr, path::Path};
use tokio::{fs, process::Command, signal::ctrl_c};

use crate::{
    package_json::{serialize_package_json_env, PackageJson},
    CompatMode,
};

#[must_use]
fn make_package_prefix(package: &PackageJson) -> String {
    let mut prefix = String::new();

    if let Some(name) = &package.name {
        prefix.push_str(&name.magenta().to_string());

        if let Some(version) = &package.version {
            let mut version_str = String::new();
            version_str.push('@');
            version_str.push_str(version);
            prefix.push_str(&version_str.magenta().dimmed().to_string());
        }

        prefix.push('\n');
    }

    prefix.push_str(&'$'.cyan().dimmed().to_string());
    prefix
}

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

#[allow(clippy::too_many_lines)]
pub async fn run_script(
    package_path: &Path,
    package_data: &PackageJson,
    script_name: &str,
    script_cmd: &str,
    extra_args: Option<&Vec<String>>,
    patched_path: &OsStr,
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

    eprintln!(
        "{} {}",
        make_package_prefix(package_data),
        full_cmd.dimmed()
    );

    let mut subproc = make_shell_cmd()?;
    subproc.current_dir(package_folder).arg(&full_cmd);

    let self_exe = env::current_exe()?;

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

    tokio::try_join! {
        async {
            if let Ok(status) = child.wait().await {
                std::process::exit(status.code().unwrap_or(1));
            }

            anyhow::Ok(())
        },

        async {
            if ctrl_c().await.is_ok() {
                if let Some(pid) = pid {
                    kill(pid, Signal::SIGINT)?;
                }
            }

            anyhow::Ok(())
        },
    }?;

    Ok(())
}
