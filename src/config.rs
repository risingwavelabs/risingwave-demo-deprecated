use std::{collections::HashMap, time::Duration};

use serde::Deserialize;

use crate::rand::DataType;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FormatType {
    Json,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub total: u64,
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

    pub timestamp: Option<TimestampConfig>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct TimestampConfig {
    /// The time is (0, `before_less_than`] before now().
    /// The duration is randomly generated on every record. So the records are not in order by time.
    #[serde(with = "humantime_serde")]
    pub before_less_than: Duration,
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
              before_less_than: 1s
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
                "click_timestamp".to_string() => FieldConfig {
                    data_type: DataType::Timestamp,
                    enum_variants: vec![],
                    timestamp: Some(TimestampConfig {
                        before_less_than: Duration::from_secs(1),
                    }),
                },
                "bar".to_string() => FieldConfig {
                    data_type: DataType::String,
                    enum_variants: vec![],
                    timestamp: None,
                },
                "platform".to_string() => FieldConfig {
                    data_type: DataType::Enum,
                    enum_variants: vec!["ios".to_string(), "android".to_string()],
                    timestamp: None,
                }
            }
        );
    }
}
