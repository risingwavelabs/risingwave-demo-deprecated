use crate::config::{Config, DataType, FormatType};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Generator {
    config: Config,
}

impl Generator {
    pub fn new(config: Config) -> Self {
        Self { config }
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
            .map(|(key, d)| (key.clone(), Self::new_json_field(d.clone())))
            .collect();
        serde_json::to_string(&record).unwrap()
    }

    fn new_json_field(data_type: DataType) -> serde_json::Value {
        use crate::rand::*;
        match data_type {
            DataType::StringZh => json!(rand_string_zh()),
            DataType::String => json!(rand_string()),
            DataType::Enum(variants) => json!(rand_enum(&variants)),
            DataType::Long(cfg) => json!(rand_long(cfg)),
            DataType::Int(cfg) => json!(rand_int(cfg)),
            DataType::Float(cfg) => json!(rand_float(cfg)),
            DataType::Timestamp(cfg) => json!(rand_timestamp(&cfg)),
            DataType::Name => json!(rand_name()),
        }
    }
}
