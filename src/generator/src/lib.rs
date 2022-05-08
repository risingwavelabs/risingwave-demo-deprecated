use anyhow::anyhow;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub mod json_generator;
pub mod json_template;

pub fn load_json_template(path: String) -> anyhow::Result<HashMap<String, String>> {
    let file = File::open(path).map_err(|e| anyhow::format_err!(e))?;
    let reader = BufReader::new(file);
    let config_json: HashMap<String, String> = serde_json::from_reader(reader).unwrap();
    Ok(config_json)
}

pub fn load_toml_config<'a, T: serde::Deserialize<'a>>(toml_str: &'a str) -> anyhow::Result<T> {
    toml::from_str(toml_str).map_err(|_| anyhow!("can't read toml file"))
}

// get current config path. only for test
pub fn get_config_path(config_file: &str) -> String {
    let current_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let parent = current_path.parent().unwrap().parent().unwrap();
    let path_string = parent.as_os_str().to_str().unwrap().to_string();
    path_string + "/configs/" + config_file
}
