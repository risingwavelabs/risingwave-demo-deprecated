use serde::Deserialize;
use std::collections::HashMap;
use validator::Validate;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FormatType {
    Json,
    Sql(SqlConfig),
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct SqlConfig {
    pub table: String,
}

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct Config {
    pub total: u64,
    pub format: FormatType,
    /// Name => [`ConnectorConfig`].
    pub connectors: HashMap<String, ConnectorConfig>,
    pub qps: Option<u32>,
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

#[cfg(test)]
mod tests {
    use crate::config::FormatType;

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
        ";
        let cfg: Config = serde_yaml::from_str(s).unwrap();
        assert_eq!(cfg.total, 10);
        assert_eq!(cfg.format, FormatType::Json);
    }
}
