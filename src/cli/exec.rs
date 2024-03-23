use color_eyre::eyre::Result;
use owo_colors::{OwoColorize as _, Stream};
use std::fs;
use std::path::PathBuf;

use crate::package_json::PackageJson;
use crate::run::{has_exec, run_exec};

use super::ExecArgs;

pub fn handle(package_paths: impl Iterator<Item = PathBuf>, args: &ExecArgs) -> Result<()> {
    let mut executed_exec = false;

    for package_path in package_paths {
        if let Ok(raw) = fs::read_to_string(&package_path) {
            if let Ok(package) = serde_json::from_str::<PackageJson>(&raw) {
                if has_exec(&package_path, &args.bin) {
                    eprint!(
                        "{}",
                        package.make_prefix(
                            match crate::get_level() {
                                1 => None,
                                _ => Some(&args.bin),
                            },
                            Stream::Stderr
                        )
                    );

                    run_exec(&package_path, &package, &args.bin, &args.extra_args)?;

                    executed_exec = true;
                    break;
                }
            }
        }
    }

    if !executed_exec {
        eprintln!(
            "{}  No binary found with name {}.",
            "error".if_supports_color(Stream::Stderr, |text| text.red()),
            args.bin
                .if_supports_color(Stream::Stderr, |text| text.bold()),
        );

        std::process::exit(1);
    }

    Ok(())
}
