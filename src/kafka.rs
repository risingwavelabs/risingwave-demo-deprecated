use crate::config::{Config, ConnectorConfig, KafkaConfig};
use crate::generator::Generator;
use rdkafka::error::KafkaError;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

#[derive(Debug)]
pub struct Producer {
    kafka_props: KafkaConfig,
    generator: Generator,
}

/// Send message to kafka topic (data format dependencies config file)
impl Producer {
    pub fn new(cfg: Config) -> Self {
        let generator = Generator::new(cfg.clone());
        let kafka_props = match &cfg.connector {
            ConnectorConfig::Kafka(kafka_props) => kafka_props.clone(),
        };
        Self {
            kafka_props,
            generator,
        }
    }

    pub async fn run(&self) {
        let topic = &self.kafka_props.topic;
        let producer = self.rdkafka_producer();

        let mut counter = 0_i32;
        loop {
            let msg = self.generator.generate();
            match self.send_record(&producer, &msg).await {
                Err(e) => {
                    println!(
                        "ERROR: failed to send message to kafka (topic={}, msg={}): {}",
                        topic, &msg, e
                    );
                    break;
                }
                Ok(_) => {
                    if counter % 100 == 0 && counter > 0 {
                        println!("{} {} messages sent successfully", topic, counter);
                    }
                    counter += 1;
                }
            }
        }
    }

    async fn send_record(
        &self,
        producer: &FutureProducer,
        msg: &str,
    ) -> std::result::Result<(), KafkaError> {
        producer
            .send::<Vec<u8>, _, _>(
                FutureRecord::to(&self.kafka_props.topic).payload(&msg.as_bytes().to_vec()),
                Duration::from_secs(0),
            )
            .await
            .map_err(|(e, _)| e)
            .map(|_| ())
    }

    fn rdkafka_producer(&self) -> FutureProducer {
        rdkafka::ClientConfig::new()
            .set("bootstrap.servers", self.kafka_props.broker.as_str())
            .set(
                "message.timeout.ms",
                self.kafka_props.timeout_ms.to_string(),
            )
            .create()
            .expect("can't create kafka producer")
    }
}
