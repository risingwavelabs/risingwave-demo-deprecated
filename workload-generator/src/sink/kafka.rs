use crate::config::KafkaConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

pub struct KafkaSink {
    cfg: KafkaConfig,
    producer: FutureProducer,
}

impl KafkaSink {
    pub fn new(cfg: KafkaConfig) -> Self {
        let producer = Self::rdkafka_producer(&cfg);
        Self { cfg, producer }
    }

    pub async fn send_record(&self, msg: &str) -> anyhow::Result<()> {
        self.producer
            .send::<Vec<u8>, _, _>(
                FutureRecord::to(&self.cfg.topic).payload(&msg.as_bytes().to_vec()),
                Duration::from_secs(0),
            )
            .await
            .map_err(|(e, _)| anyhow::format_err!(e))
            .map(|_| ())
    }

    fn rdkafka_producer(cfg: &KafkaConfig) -> FutureProducer {
        rdkafka::ClientConfig::new()
            .set("bootstrap.servers", cfg.broker.as_str())
            .set("message.timeout.ms", cfg.timeout_ms.to_string())
            .set_log_level(rdkafka::config::RDKafkaLogLevel::Error)
            .create()
            .expect("can't create kafka producer")
    }
}
