use crate::json::json_generator::RunningState::Process;
use crate::json::json_template::{
    FloatValue, IntValue, JsonDataType, LongValue, RandomStringValue, Value,
};
use rand::Rng;
use serde_derive::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use tokio::sync;

#[derive(Debug, Deserialize, Clone)]
pub struct GeneratorRuleConfig {
    generator: HashMap<String, JsonNoNestedRule>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JsonNoNestedRule {
    total: i32,
    cardinal: Vec<FieldCardinal>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct FieldCardinal {
    field: String,
    count: i32,
}

#[derive(Debug, Clone)]
pub enum RunningState {
    Process,
    Success,
    Failure,
}

#[derive(Debug, Clone)]
pub struct JsonGenerator {
    type_mapping: HashMap<String, JsonDataType>,
    rule: JsonNoNestedRule,
}

impl JsonGenerator {
    pub fn new(template: HashMap<String, String>, rule: JsonNoNestedRule) -> Self {
        let mut type_mapping: HashMap<String, JsonDataType> = HashMap::new();
        for (k, v) in template.iter() {
            type_mapping.insert(k.clone(), JsonDataType::from_str(v).unwrap());
        }
        Self { type_mapping, rule }
    }

    pub async fn batch_generate(
        &self,
        data_tx: sync::mpsc::Sender<String>,
        notify_tx: sync::watch::Sender<RunningState>,
    ) -> anyhow::Result<()> {
        if let Err(err) = notify_tx.send(Process) {
            Err(anyhow::Error::from(err))
        } else {
            // TODO: instead of using the State trait
            let mut rule_state: HashMap<String, Vec<serde_json::Value>> = HashMap::new();
            let field_rule_vec = self.rule.cardinal.clone();
            for _ in 0..self.rule.total {
                let mut record = self.new_json_object();
                field_rule_vec.iter().for_each(|field_cardinal| {
                    let field = field_cardinal.field.clone();
                    if rule_state.contains_key(field.as_str()) {
                        let values = rule_state.get(field.as_str()).unwrap();
                        let values_size = values.len() as i32;
                        if values_size > field_cardinal.count {
                            let idx = rand::thread_rng().gen_range(0..values_size) as usize;
                            let old_value = values.get(idx).unwrap().clone();
                            record.insert(field, old_value);
                        }
                    } else {
                        rule_state.insert(
                            field.clone(),
                            vec![record.get(field.as_str()).unwrap().clone()],
                        );
                    }
                });
                let json_string = serde_json::to_string(&record).unwrap();
                if let Err(_err) = data_tx.send(json_string).await {
                    let _send_ignore = notify_tx.send(RunningState::Failure);
                }
            }
            let _ignore_send = notify_tx.send(RunningState::Success);
            Ok(())
        }
    }

    pub fn new_json_object(&self) -> HashMap<String, serde_json::Value> {
        let tuples: Vec<(String, serde_json::Value)> = self
            .type_mapping
            .iter()
            .map(|(key, json_type)| {
                let json_value = match json_type.clone() {
                    JsonDataType::StringZh => {
                        json!(RandomStringValue {}.get_value(Some(JsonDataType::StringZh)))
                    }
                    JsonDataType::String => {
                        json!(RandomStringValue {}.get_value(Some(JsonDataType::String)))
                    }
                    JsonDataType::Enum => {
                        json!(RandomStringValue {}.get_value(Some(JsonDataType::Enum)))
                    }
                    JsonDataType::Long => {
                        json!(LongValue {}.get_value(Some(JsonDataType::Long)))
                    }
                    JsonDataType::Int => {
                        json!(IntValue {}.get_value(None))
                    }
                    JsonDataType::Float => {
                        json!(FloatValue {}.get_value(None))
                    }
                    JsonDataType::Timestamp => {
                        json!(LongValue {}.get_value(Some(JsonDataType::Timestamp)))
                    }
                };
                (key.clone(), json_value)
            })
            .collect();
        HashMap::from_iter(tuples)
    }
}

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

pub fn load_generator_config(path: String) -> anyhow::Result<GeneratorRuleConfig> {
    let config_string = std::fs::read_to_string(path).expect("can't read toml file");
    let config_rs = toml::from_str(&config_string);
    match config_rs {
        Ok(generator) => Ok(generator),
        Err(err) => Err(anyhow::Error::from(err)),
    }
}

#[cfg(test)]
mod tests {
    use crate::json::json_generator::{load_generator_config, load_json_template, JsonGenerator};
    use std::path::PathBuf;

    fn get_config_path(config_file: &str) -> String {
        let current_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let parent = current_path.parent().unwrap().parent().unwrap();
        let path_string = parent.as_os_str().to_str().unwrap().to_string();
        path_string + "/configs/" + config_file
    }

    #[test]
    fn get_json_generator_rule() {
        let rule_config_path = get_config_path("generator.toml");
        let rule_rs = load_generator_config(rule_config_path);
        assert!(rule_rs.is_ok());
    }

    #[test]
    fn test_load_json_template() {
        let template_file_path = get_config_path("json-nonested.json");
        let json_template_rs = load_json_template(template_file_path);
        assert!(json_template_rs.is_ok());
    }

    #[test]
    fn test_generator() {
        let template_file_path = get_config_path("json-nonested.json");
        let json_template = load_json_template(template_file_path).unwrap();
        println!("json_template = {:?}", json_template);
        let rule_config_path = get_config_path("generator.toml");
        let rule = load_generator_config(rule_config_path).unwrap();
        println!("rule_config = {:?}", rule);
        let json_generator = JsonGenerator::new(
            json_template,
            rule.generator.get("jsonnonested").unwrap().clone(),
        );
        for _ in 0..3 {
            let json_obj = json_generator.new_json_object();
            assert!(!json_obj.is_empty());
            println!(
                "json_string = {}",
                serde_json::to_string(&json_obj).unwrap()
            );
        }
    }
}
