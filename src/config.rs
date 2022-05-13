use std::{collections::HashMap, time::Duration};

use serde::{Deserialize, Serialize};

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
    pub schema: HashMap<String, DataType>,
}

fn validate_schema(_schema: &HashMap<String, DataType>) -> Result<(), ValidationError> {
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    StringZh,
    String,
    Name,
    Enum(Vec<String>),
    Long(Option<LongConfig>),
    Int(Option<IntConfig>),
    Float(Option<FloatConfig>),
    Timestamp(Option<TimestampConfig>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Validate)]
pub struct LongConfig {
    pub start: i64,
    pub stop: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Validate)]
pub struct IntConfig {
    pub start: i32,
    pub stop: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Validate)]
pub struct FloatConfig {
    pub start: f64,
    pub stop: f64,
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

    use crate::config::{DataType, FormatType, IntConfig, TimestampConfig};

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
            timestamp:
              random_delay: 1s
          bar: string
          foo: 
            int:
              start: 1
              stop: 10
          platform:
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
                "click_timestamp".to_string() => DataType::Timestamp(Some(TimestampConfig {
                        random_delay:  Duration::from_secs(1),
                })),
                "bar".to_string() =>  DataType::String,
                "foo".to_string() => DataType::Int(Some(IntConfig {
                    start: 1,
                    stop: 10,
                })),
                "platform".to_string() => DataType::Enum(
                    vec!["ios".to_string(), "android".to_string()]
                ),
            }
        );
    }
}
