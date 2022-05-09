pub mod kafka;

use crate::{
    config::{Config, ConnectorConfig},
    generator::Generator,
};

pub enum Sink {
    Kafka(kafka::KafkaSink),
}

impl Sink {
    pub fn new(cfg: Config) -> Self {
        match &cfg.connector {
            ConnectorConfig::Kafka(kafka_cfg) => {
                Self::Kafka(kafka::KafkaSink::new(kafka_cfg.clone()))
            }
        }
    }

    pub async fn send_record(&self, msg: &str) -> anyhow::Result<()> {
        match self {
            Self::Kafka(kafka_sink) => kafka_sink.send_record(msg).await,
        }
    }
}

/// Loop until all total records are sent or a failure occurs.
pub async fn run_loop(cfg: Config) {
    let generator = Generator::new(cfg.clone());
    let sink = Sink::new(cfg.clone());

    let mut counter = 0_usize;
    while counter <= cfg.total {
        let msg = generator.generate();
        match sink.send_record(&msg).await {
            Err(e) => {
                println!("ERROR: failed to send message: {}\n{}", e, &msg);
                break;
            }
            Ok(_) => {
                if counter % 100 == 0 && counter > 0 {
                    println!("{} messages sent successfully", counter);
                }
                counter += 1;
            }
        }
    }
}
