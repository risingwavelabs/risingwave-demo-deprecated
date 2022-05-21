pub mod kafka;

use crate::config::{Config, ConnectorConfig};
use std::collections::HashMap;

pub enum Sink {
    Kafka(kafka::KafkaSink),
    Stdout,
}

impl Sink {
    pub async fn new(cfg: Config) -> HashMap<String, Sink> {
        let mut m = HashMap::<String, Sink>::new();
        for (name, c) in cfg.connectors {
            let sink = match c {
                ConnectorConfig::Kafka(kafka_cfg) => {
                    Self::Kafka(kafka::KafkaSink::new(kafka_cfg.clone()).await)
                }
                ConnectorConfig::Stdout => Self::Stdout,
            };
            m.insert(name.clone(), sink);
        }
        m
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
