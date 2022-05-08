use crate::json_template::{
    FloatValue, IntValue, JsonDataType, LongValue, RandomStringValue, Value,
};
use rand::Rng;
use serde_derive::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::str::FromStr;
use tokio::sync;

#[derive(Debug, Deserialize, Clone)]
pub struct GeneratorRuleConfig {
    pub generator: HashMap<String, JsonNoNestedRule>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JsonNoNestedRule {
    total: i32,
    cardinal: Vec<FieldCardinal>,
}

#[derive(Debug, Deserialize, Clone)]
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
        if let Err(err) = notify_tx.send(RunningState::Process) {
            Err(anyhow::Error::from(err))
        } else {
            // TODO: instead of using the State trait
            let mut rule_state: HashMap<String, Vec<serde_json::Value>> = HashMap::new();
            let field_rule_vec = self.rule.cardinal.clone();
            for count in 0..=self.rule.total {
                let mut record = self.new_json_object();
                field_rule_vec.iter().for_each(|field_cardinal| {
                    let field = field_cardinal.field.clone();
                    if rule_state.contains_key(field.as_str()) {
                        let values = rule_state.get(field.as_str()).unwrap();
                        let values_size = values.len() as i32;
                        if values_size >= field_cardinal.count {
                            let idx = rand::thread_rng().gen_range(0..values_size) as usize;
                            let old_value = values.get(idx).unwrap().clone();
                            record.insert(field.clone(), old_value);
                        } else {
                            let field_value = record.get(field.as_str()).unwrap();
                            let mut values =
                                rule_state.get(field.clone().as_str()).unwrap().clone();
                            values.push(field_value.clone());
                            rule_state.insert(field.clone(), values);
                        }
                    } else {
                        rule_state.insert(
                            field.clone(),
                            vec![record.get(field.as_str()).unwrap().clone()],
                        );
                    }
                });
                let json_string = serde_json::to_string(&record).unwrap();
                let send_rs = data_tx.send(json_string).await;
                match send_rs {
                    Ok(()) => {
                        if count % 100 == 0 && count > 0 {
                            println!("generate message success. count = {}", count);
                        }
                    }
                    Err(send_err) => {
                        println!("send message error = {:?}", send_err);
                        let _send_ignore = notify_tx.send(RunningState::Failure);
                    }
                }
            }
            println!("all message send complete.");
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

#[cfg(test)]
mod tests {
    use crate::json_generator::{GeneratorRuleConfig, JsonGenerator, RunningState};
    use crate::{get_config_path, load_json_template, load_toml_config};

    #[test]
    fn test_load_generator_rule() {
        let rule_config_path = get_config_path("generator.toml");
        let read_toml_rs = std::fs::read_to_string(rule_config_path).unwrap();
        let rule_rs = load_toml_config::<GeneratorRuleConfig>(read_toml_rs.as_str());
        assert!(rule_rs.is_ok());
    }

    #[test]
    fn test_load_json_template() {
        let template_file_path = get_config_path("json-nonested.json");
        let json_template_rs = load_json_template(template_file_path);
        assert!(json_template_rs.is_ok());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_batch_generate() {
        let json_generator = new_generate();
        let (tx, mut rx) = tokio::sync::mpsc::channel(10);
        let (notify_tx, mut notify_rx) = tokio::sync::watch::channel(RunningState::Process);
        let wait_join = tokio::task::spawn(async move {
            loop {
                tokio::select! {
                   data = rx.recv() => {
                      println!("receive data {}", data.unwrap());
                   },
                   run_status = notify_rx.changed() => {
                        if run_status.is_ok() {
                            if let RunningState::Success = *notify_rx.borrow() {
                                println!("receive RunningState is Success.");
                                break;
                            }
                        }
                   }
                }
            }
        });
        let batch_gen_rs = json_generator.batch_generate(tx, notify_tx).await;
        assert!(batch_gen_rs.is_ok());
        let rs = wait_join.await;
        assert!(rs.is_ok())
    }

    fn new_generate() -> JsonGenerator {
        let template_file_path = get_config_path("json-nonested.json");
        let json_template = load_json_template(template_file_path).unwrap();
        println!("json_template = {:?}", json_template);
        let rule_config_path = get_config_path("generator.toml");

        let read_toml_rs = std::fs::read_to_string(rule_config_path).unwrap();

        let rule: GeneratorRuleConfig = load_toml_config(read_toml_rs.as_str()).unwrap();
        println!("rule_config = {:?}", rule);
        JsonGenerator::new(
            json_template,
            rule.generator.get("jsonnonested").unwrap().clone(),
        )
    }

    #[test]
    fn test_new_json_object() {
        let json_generator = new_generate();
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
