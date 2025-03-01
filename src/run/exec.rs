use color_eyre::Result;
use std::{env, path::Path, process::Command};

use crate::{
    cli::ExecArgs,
    package_json::PackageJson,
    util::{itoa, NRR_LEVEL},
};

use super::util::make_patched_path;

pub fn exec(package_path: &Path, package_data: &PackageJson, args: &ExecArgs) -> Result<()> {
    let package_folder = package_path.parent().unwrap();

    let mut subproc = Command::new(&args.executable);
    subproc.current_dir(package_folder).args(&args.args);

    if let Some(env_file) = &args.env_file {
        subproc.envs(env_file.iter());
    }

    subproc
        .env("PATH", make_patched_path(package_path)?)
        .env("__NRR_LEVEL", itoa(*NRR_LEVEL + 1));

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
        use std::os::unix::process::CommandExt as _;
        Err(subproc.exec())?;
        Ok(())
    }

    #[cfg(windows)]
    {
        let _ = ctrlc::set_handler(|| {});
        let status = subproc.status()?;
        std::process::exit(status.code().unwrap_or(1));
    }
}
