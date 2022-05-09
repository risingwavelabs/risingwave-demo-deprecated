use std::collections::HashMap;

use serde::Deserialize;

use crate::rand::DataType;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FormatType {
    Json,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub total: usize,
    pub format: FormatType,
    pub connector: ConnectorConfig,
    pub schema: HashMap<String, FieldConfig>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ConnectorConfig {
    Kafka(KafkaConfig),
}

#[derive(Debug, Deserialize, Clone)]
pub struct KafkaConfig {
    pub broker: String,
    pub topic: String,
    pub timeout_ms: u64,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct FieldConfig {
    #[serde(rename = "type")]
    pub data_type: DataType,

    #[serde(rename = "enum", default)]
    pub enum_variants: Vec<String>,
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{
        config::{FieldConfig, FormatType},
        rand::DataType,
    };

    use super::Config;

    #[test]
    fn test_load_yaml() {
        let s = "
        total: 10
        format: json
        connector:
          kafka:
            broker: localhost:29092
            topic: test_topic
            timeout_ms: 5000
        schema:
          foo:
            type: int
          bar:
            type: string
          platform:
            type: enum
            enum:
            - ios
            - android
        ";
        let cfg: Config = serde_yaml::from_str(s).unwrap();
        assert_eq!(cfg.total, 10);
        assert_eq!(cfg.format, FormatType::Json);

        assert_eq!(
            cfg.schema,
            maplit::hashmap! {
                "foo".to_string() => FieldConfig {
                    data_type: DataType::Int,
                    enum_variants: vec![],
                },
                "bar".to_string() => FieldConfig {
                    data_type: DataType::String,
                    enum_variants: vec![],
                },
                "platform".to_string() => FieldConfig {
                    data_type: DataType::Enum,
                    enum_variants: vec!["ios".to_string(), "android".to_string()],
                }
            }
        );
    }
}
