use owo_colors::OwoColorize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    pub name: Option<String>,
    pub version: Option<String>,
    #[serde(default)]
    pub scripts: HashMap<String, String>,
}

#[must_use]
fn normalize_ser_path(path: &str) -> String {
    path.replace(|ch: char| !ch.is_alphanumeric(), "_")
}

fn serialize_into(data: &Value, path: &str, to: &mut HashMap<String, String>) {
    match data {
        Value::Null => {}
        Value::Bool(v) => {
            to.insert(normalize_ser_path(path), v.to_string());
        }
        Value::Number(v) => {
            to.insert(normalize_ser_path(path), v.to_string());
        }
        Value::String(v) => {
            to.insert(normalize_ser_path(path), v.clone());
        }
        Value::Array(v) => {
            for (idx, item) in v.iter().enumerate() {
                let new_path = path.to_owned() + "_" + &idx.to_string();
                serialize_into(item, &new_path, to);
            }
        }
        Value::Object(v) => {
            for (key, value) in v {
                let new_path = path.to_owned() + "_" + key;
                serialize_into(value, &new_path, to);
            }
        }
    }
}

#[must_use]
pub fn serialize_package_json_env(data: &Value) -> HashMap<String, String> {
    let mut ret: HashMap<String, String> = HashMap::new();
    serialize_into(data, "npm_package", &mut ret);
    ret
}

#[must_use]
pub fn make_package_prefix(package: &PackageJson, script_name: Option<&str>) -> String {
    let mut prefix = String::new();

    if let Some(name) = &package.name {
        prefix.push_str(&name.magenta().to_string());

        if let Some(version) = &package.version {
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
