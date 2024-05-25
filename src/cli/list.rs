use owo_colors::{OwoColorize as _, Stream};

use crate::package_json::PackageJson;
use std::path::PathBuf;

pub fn handle(package_paths: impl Iterator<Item = PathBuf>) -> bool {
    let mut found_package = false;

    for package_path in package_paths {
        if let Some(package) = PackageJson::from_path_safe(&package_path) {
            if found_package {
                println!();
            }

            print!("{}", package.make_prefix(None, Stream::Stdout));
            found_package = true;

            for (script_name, script_content) in &package.scripts {
                println!(
                    "{}",
                    script_name.if_supports_color(Stream::Stdout, |text| text.cyan())
                );
                println!("  {script_content}");
            }
        }
    }

    found_package
}
