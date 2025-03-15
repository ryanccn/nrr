use std::{env, path::Path};

use color_eyre::Result;
use owo_colors::{OwoColorize as _, Stream};

use crate::{
    cli::SharedRunOptions,
    package_json::PackageJson,
    util::{ExitCode, NRR_LEVEL, itoa, signals},
};

use super::util::{make_patched_path, make_shell_cmd};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ScriptType {
    Pre,
    Post,
    Normal,
}

impl ScriptType {
    #[must_use]
    pub fn prefix(self) -> String {
        match self {
            Self::Normal => String::new(),
            Self::Pre => "pre".into(),
            Self::Post => "post".into(),
        }
    }
}

fn single_script(
    script_type: ScriptType,
    package_path: &Path,
    package_data: &PackageJson,
    script_name: &str,
    script_cmd: &str,
    options: &SharedRunOptions,
) -> Result<()> {
    let package_folder = package_path.parent().unwrap();

    let mut full_cmd = script_cmd.to_owned();

    if script_type == ScriptType::Normal {
        options
            .args
            .iter()
            .map(|f| shlex::try_quote(f))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .for_each(|arg| {
                full_cmd.push(' ');
                full_cmd.push_str(arg.as_ref());
            });
    }

    if !options.silent {
        let cmd_prefix = script_type.prefix() + &"$".repeat(*NRR_LEVEL);

        eprintln!(
            "{} {}",
            cmd_prefix
                .if_supports_color(Stream::Stderr, |text| text.cyan())
                .if_supports_color(Stream::Stderr, |text| text.dimmed()),
            full_cmd.if_supports_color(Stream::Stderr, |text| text.dimmed())
        );
    }

    let mut command = make_shell_cmd();
    command.current_dir(package_folder).arg(&full_cmd);

    if let Some(env_file) = &options.env_file {
        command.envs(env_file.iter());
    }

    command
        .env("PATH", make_patched_path(package_path)?)
        .env("__NRR_LEVEL", itoa(*NRR_LEVEL + 1));

    command
        .env("npm_execpath", env::current_exe()?)
        .env("npm_lifecycle_event", script_name)
        .env("npm_lifecycle_script", &full_cmd);

    command.env("npm_package_json", package_path);

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

    if !status.success() {
        eprintln!(
            "{}  Script {} exited with code {}!",
            "error".if_supports_color(Stream::Stderr, |text| text.red()),
            script_name.if_supports_color(Stream::Stderr, |text| text.bold()),
            status.code().unwrap_or(1)
        );

        return Err(ExitCode(status.code().unwrap_or(1)).into());
    }

    Ok(())
}

pub fn script(
    package_path: &Path,
    package_data: &PackageJson,
    script_name: &str,
    script_cmd: &str,
    options: &SharedRunOptions,
) -> Result<()> {
    if !options.silent {
        eprint!(
            "{}",
            package_data.make_prefix(
                match *NRR_LEVEL {
                    1 => None,
                    _ => Some(script_name),
                },
                Stream::Stderr
            )
        );
    }

    if !options.no_pre_post {
        let pre_script_name = String::from("pre") + script_name;

        if let Some(pre_script_cmd) = package_data.scripts.get(&pre_script_name) {
            single_script(
                ScriptType::Pre,
                package_path,
                package_data,
                &pre_script_name,
                pre_script_cmd,
                options,
            )?;
        }
    }

    single_script(
        ScriptType::Normal,
        package_path,
        package_data,
        script_name,
        script_cmd,
        options,
    )?;

    if !options.no_pre_post {
        let post_script_name = String::from("post") + script_name;

        if let Some(post_script_cmd) = package_data.scripts.get(&post_script_name) {
            single_script(
                ScriptType::Post,
                package_path,
                package_data,
                &post_script_name,
                post_script_cmd,
                options,
            )?;
        }
    }

    Ok(())
}
