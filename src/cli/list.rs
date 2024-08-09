use color_eyre::eyre::Result;
use owo_colors::{OwoColorize as _, Stream};
use terminal_size::terminal_size;
use unicode_width::UnicodeWidthStr as _;

use crate::package_json::PackageJson;
use std::{
    io::{stdout, Write as _},
    path::PathBuf,
};

pub fn handle(package_paths: impl Iterator<Item = PathBuf>) -> Result<bool> {
    let mut found_package = false;

    for package_path in package_paths {
        if let Some(package) = PackageJson::from_path_safe(&package_path) {
            let mut lock = stdout().lock();

            if found_package {
                lock.write_all("\n".as_bytes())?;
            }

            lock.write_all(package.make_prefix(None, Stream::Stdout).as_bytes())?;
            found_package = true;

            let longest_pad = package
                .scripts
                .iter()
                .map(|s| s.0.width())
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
                            lock.write_all(" ".repeat(longest_pad + 2).as_bytes())?;
                        }
                        lock.write_all(line.as_bytes())?;
                        lock.write_all("\n".as_bytes())?;
                    }
                } else {
                    lock.write_all(content.as_bytes())?;
                    lock.write_all("\n".as_bytes())?;
                }
            }
        }
    }

    Ok(found_package)
}
