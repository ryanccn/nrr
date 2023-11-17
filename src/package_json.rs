use std::collections::HashMap;

use serde_json::Value;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    pub name: Option<String>,
    pub version: Option<String>,
    #[serde(default)]
    pub scripts: HashMap<String, String>,
}

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

pub fn serialize_package_json_env(data: &Value) -> HashMap<String, String> {
    let mut ret: HashMap<String, String> = HashMap::new();
    serialize_into(data, "npm_package", &mut ret);
    ret
}
