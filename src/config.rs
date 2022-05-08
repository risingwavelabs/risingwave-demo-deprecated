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
    pub schema: Vec<FieldConfig>,
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
    pub field: String,

    #[serde(rename = "type")]
    pub data_type: DataType,
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
        schema:
            - field: foo
              type: int
            - field: bar
              type: string
        ";
        let cfg: Config = serde_yaml::from_str(s).unwrap();
        assert_eq!(cfg.total, 10);
        assert_eq!(cfg.format, FormatType::Json);

        assert_eq!(
            cfg.schema,
            vec![
                FieldConfig {
                    field: "foo".to_string(),
                    data_type: DataType::Int,
                },
                FieldConfig {
                    field: "bar".to_string(),
                    data_type: DataType::String
                }
            ]
        );
    }
}
