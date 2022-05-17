use crate::config::KafkaConfig;
use rdkafka::{
    admin::{AdminClient, AdminOptions, NewTopic, TopicReplication},
    client::DefaultClientContext,
    config::FromClientConfig,
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};
use std::time::Duration;

pub struct KafkaSink {
    cfg: KafkaConfig,
    producer: FutureProducer,
}

impl KafkaSink {
    pub async fn new(cfg: KafkaConfig) -> Self {
        let producer = Self::rdkafka_producer(&cfg);
        Self::create_topic(&cfg).await;
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

    fn rdkafka_config(cfg: &KafkaConfig) -> ClientConfig {
        rdkafka::ClientConfig::new()
            .set("bootstrap.servers", cfg.broker.as_str())
            .set("message.timeout.ms", cfg.timeout_ms.to_string())
            .set_log_level(rdkafka::config::RDKafkaLogLevel::Info)
            .clone()
    }

    fn rdkafka_producer(cfg: &KafkaConfig) -> FutureProducer {
        Self::rdkafka_config(cfg)
            .create()
            .expect("failed to create kafka producer")
    }

    async fn create_topic(cfg: &KafkaConfig) {
        let admin = AdminClient::<DefaultClientContext>::from_config(&Self::rdkafka_config(cfg))
            .expect("failed to create kafka admin client");
        admin
            .create_topics(
                &[NewTopic::new(
                    cfg.topic.as_str(),
                    1,
                    TopicReplication::Fixed(1),
                )],
                &AdminOptions::default(),
            )
            .await
            .unwrap_or_else(|_| panic!("failed to create topic: {}", cfg.topic));
    }
}
