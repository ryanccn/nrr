use color_eyre::eyre::Result;
use owo_colors::{OwoColorize as _, Stream};
use terminal_size::terminal_size;

use crate::package_json::PackageJson;
use std::{
    io::{stdout, Write as _},
    path::PathBuf,
};

pub fn handle(package_paths: impl Iterator<Item = PathBuf>) -> Result<bool> {
    let mut found_package = false;

    for package_path in package_paths {
        if let Some(package) = PackageJson::from_path_safe(&package_path) {
            if found_package {
                println!();
            }

            let mut lock = stdout().lock();

            write!(lock, "{}", package.make_prefix(None, Stream::Stdout))?;
            found_package = true;

            let longest_pad = package
                .scripts
                .iter()
                .map(|s| s.0.len())
                .max()
                .unwrap_or_default();

            let terminal_width = terminal_size().map(|size| size.0 .0 as usize);

            for (name, content) in &package.scripts {
                write!(
                    lock,
                    "{}  ",
                    format!("{name:>longest_pad$}")
                        .if_supports_color(Stream::Stdout, |text| text.cyan())
                )?;

                if let Some(terminal_width) = terminal_width {
                    let wrapped = content.chars().collect::<Vec<_>>();

                    for (idx, line) in wrapped
                        .as_slice()
                        .chunks(terminal_width - longest_pad - 2)
                        .map(|s| s.iter().collect::<String>())
                        .enumerate()
                    {
                        if idx != 0 {
                            write!(lock, "{}", " ".repeat(longest_pad + 2))?;
                        }
                        writeln!(lock, "{line}")?;
                    }
                } else {
                    writeln!(lock, "{content}")?;
                }
            }
        }
    }

    Ok(found_package)
}
