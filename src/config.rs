use std::{collections::HashMap, time::Duration};

use serde::{Deserialize, Serialize};

use crate::rand::DataType;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FormatType {
    Json,
}

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct Config {
    pub total: u64,
    pub format: FormatType,
    pub connector: ConnectorConfig,
    pub qps: Option<u32>,

    #[validate(custom = "validate_schema")]
    pub schema: HashMap<String, FieldConfig>,
}

fn validate_schema(schema: &HashMap<String, FieldConfig>) -> Result<(), ValidationError> {
    for (_, f) in schema.iter() {
        f.validate().map_err(|e| {
            println!("ERROR: {}", e);
            ValidationError::new("schema")
        })?;
    }
    Ok(())
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ConnectorConfig {
    Kafka(KafkaConfig),
    Stdout,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KafkaConfig {
    pub broker: String,
    pub topic: String,
    pub timeout_ms: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct FieldConfig {
    #[serde(rename = "type")]
    pub data_type: DataType,

    #[serde(rename = "enum", default)]
    pub enum_variants: Vec<String>,

    pub timestamp: Option<TimestampConfig>,

    /// The number of distinct values to generate.
    pub cardinality: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct TimestampConfig {
    /// A randomized delay within the range of (0, `random_delay`].
    /// If it's set, this field will not be guaranteed to ordered.
    #[serde(with = "humantime_serde")]
    pub random_delay: Duration,
}

#[cfg(test)]
mod tests {
    use std::{time::Duration, vec};

    use crate::{
        config::{FieldConfig, FormatType, TimestampConfig},
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
          click_timestamp:
            type: timestamp
            timestamp:
              random_delay: 1s
          bar:
            type: string
            cardinality: 100
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
                "click_timestamp".to_string() => FieldConfig {
                    data_type: DataType::Timestamp,
                    enum_variants: vec![],
                    timestamp: Some(TimestampConfig {
                        random_delay: Duration::from_secs(1),
                    }),
                    cardinality: None,
                },
                "bar".to_string() => FieldConfig {
                    data_type: DataType::String,
                    enum_variants: vec![],
                    timestamp: None,
                    cardinality: Some(100),
                },
                "platform".to_string() => FieldConfig {
                    data_type: DataType::Enum,
                    enum_variants: vec!["ios".to_string(), "android".to_string()],
                    timestamp: None,
                    cardinality: None,
                }
            }
        );
    }
}
