use anyhow::anyhow;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

mod generate_state;
mod json;

pub fn load_json_template(path: String) -> anyhow::Result<HashMap<String, String>> {
    let file_rs = File::open(path);
    match file_rs {
        Ok(file) => {
            let reader = BufReader::new(file);
            let config_json: HashMap<String, String> = serde_json::from_reader(reader).unwrap();
            Ok(config_json)
        }
        Err(err) => Err(anyhow::Error::from(err)),
    }
}

pub fn load_toml_config<'a, T: serde::Deserialize<'a>>(toml_str: &'a str) -> anyhow::Result<T> {
    let toml_read_rs = toml::from_str(toml_str);
    if let Ok(toml_obj) = toml_read_rs {
        Ok(toml_obj)
    } else {
        Err(anyhow!("can't read toml file"))
    }
}
