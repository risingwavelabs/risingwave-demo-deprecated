pub mod kafka;

use crate::config::{Config, ConnectorConfig};

pub enum Sink {
    Kafka(kafka::KafkaSink),
    Stdout,
}

impl Sink {
    pub async fn new(cfg: Config) -> Self {
        match &cfg.connector {
            ConnectorConfig::Kafka(kafka_cfg) => {
                Self::Kafka(kafka::KafkaSink::new(kafka_cfg.clone()).await)
            }
            ConnectorConfig::Stdout => Self::Stdout,
        }
    }

    pub async fn send_record(&self, msg: &str) -> anyhow::Result<()> {
        match self {
            Self::Kafka(kafka_sink) => kafka_sink.send_record(msg).await,
            Self::Stdout => {
                println!("{}", msg);
                Ok(())
            }
        }
    }
}
