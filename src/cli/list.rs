use owo_colors::{OwoColorize as _, Stream};

use crate::package_json::PackageJson;
use std::{fs, path::PathBuf};

pub fn handle(package_paths: impl Iterator<Item = PathBuf>) -> bool {
    let mut found_package = false;

    for package_path in package_paths {
        if let Ok(Ok(package)) =
            fs::read(&package_path).map(|mut raw| simd_json::from_slice::<PackageJson>(&mut raw))
        {
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
