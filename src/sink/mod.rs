pub mod kafka;

use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};

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

    let pb = ProgressBar::new(cfg.total);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
        .progress_chars("#>-"));
    for _ in (0..cfg.total).progress_with(pb) {
        let msg = generator.generate();
        match sink.send_record(&msg).await {
            Err(e) => {
                println!("ERROR: failed to send message: {}\n{}", e, &msg);
                break;
            }
            Ok(()) => (),
        }
    }
}
