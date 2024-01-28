use owo_colors::OwoColorize;
use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson<'a> {
    #[serde(borrow)]
    pub name: Option<&'a str>,
    #[serde(borrow)]
    pub version: Option<&'a str>,

    #[serde(borrow, default)]
    pub scripts: HashMap<String, &'a str>,
}

impl PackageJson<'_> {
    #[must_use]
    pub fn make_prefix(&self, script_name: Option<&str>) -> String {
        let mut prefix = String::new();

        if let Some(name) = &self.name {
            prefix.push_str(&name.magenta().to_string());

            if let Some(version) = &self.version {
                let mut version_str = String::new();
                version_str.push('@');
                version_str.push_str(version);
                prefix.push_str(&version_str.magenta().dimmed().to_string());
            }

            if let Some(script_name) = script_name {
                prefix.push(' ');
                prefix.push_str(&script_name.cyan().to_string());
            }

            prefix.push('\n');
        }

        prefix
    }
}
