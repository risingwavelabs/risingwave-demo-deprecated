use crate::config::{Config, FieldConfig, FormatType};
use rand::Rng;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Generator {
    config: Config,

    gen_json_values: HashMap<String, Vec<serde_json::Value>>,
}

impl Generator {
    pub fn new(config: Config) -> Self {
        let gen_json_values = match &config.format {
            FormatType::Json => config
                .schema
                .iter()
                .map(|(key, f)| {
                    let values = if let Some(card) = f.cardinality.as_ref() {
                        (0..*card).map(|_| Self::new_json_field(f)).collect()
                    } else {
                        vec![]
                    };
                    (key.clone(), values)
                })
                .collect(),
        };
        Self {
            gen_json_values,
            config,
        }
    }

    pub fn generate(&self) -> String {
        match &self.config.format {
            FormatType::Json => self.new_json_object(),
        }
    }

    fn new_json_object(&self) -> String {
        let record: HashMap<String, serde_json::Value> = self
            .config
            .schema
            .iter()
            .map(|(key, f)| {
                if f.cardinality.is_some() {
                    let values = self.gen_json_values.get(key).unwrap();
                    let idx = rand::thread_rng().gen_range(0..values.len());
                    return (key.clone(), values[idx].clone());
                }
                (key.clone(), Self::new_json_field(f))
            })
            .collect();
        serde_json::to_string(&record).unwrap()
    }

    fn new_json_field(f: &FieldConfig) -> serde_json::Value {
        use crate::rand::*;
        match f.data_type.clone() {
            DataType::StringZh => json!(rand_string_zh()),
            DataType::String => json!(rand_string()),
            DataType::Enum => json!(rand_enum(&f.enum_variants)),
            DataType::Long => json!(rand_long()),
            DataType::Int => json!(rand_int()),
            DataType::Float => json!(rand_float()),
            DataType::Timestamp => json!(rand_timestamp(&f.timestamp)),
        }
    }
}
