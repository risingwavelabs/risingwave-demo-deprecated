use crate::config::{Config, FormatType};
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
            FormatType::Json => {
                let record = self.new_json_object();
                serde_json::to_string(&record).unwrap()
            }
        }
    }

    fn new_json_object(&self) -> HashMap<String, serde_json::Value> {
        use crate::rand::*;

        self.config
            .schema
            .iter()
            .map(|(key, f)| {
                let json_value = match f.data_type.clone() {
                    DataType::StringZh => json!(rand_string_zh()),
                    DataType::String => json!(rand_string()),
                    DataType::Enum => json!(rand_enum(&f.enum_variants)),
                    DataType::Long => json!(rand_long()),
                    DataType::Int => json!(rand_int()),
                    DataType::Float => json!(rand_float()),
                    DataType::Timestamp => json!(rand_timestamp(&f.timestamp)),
                };
                (key.clone(), json_value)
            })
            .collect()
    }
}
