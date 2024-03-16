use color_eyre::eyre::Result;
use owo_colors::{OwoColorize, Stream};

use crate::package_json::PackageJson;
use std::{fs, path::PathBuf};

pub fn handle(package_paths: impl Iterator<Item = PathBuf>) -> Result<bool> {
    let mut found_package = false;

    for package in package_paths {
        let raw = fs::read_to_string(package)?;
        if let Ok(package_data) = serde_json::from_str::<PackageJson>(&raw) {
            print!("{}", package_data.make_prefix(None, Stream::Stdout));

            found_package = true;

            for (script_name, script_content) in &package_data.scripts {
                println!(
                    "{}",
                    script_name.if_supports_color(Stream::Stdout, |text| text.cyan())
                );
                println!("  {script_content}");
            }

            println!();
        }
    }

    Ok(found_package)
}
