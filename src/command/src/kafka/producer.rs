use data_generator::json::json_generator::{GeneratorRuleConfig, JsonGenerator, JsonNoNestedRule, RunningState};
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync;

use data_generator::{load_json_template, load_toml_config};
use serde_derive::Deserialize;

const DEFAULT_CHANNEL_BUFFER_SIZE: usize = 1000;
const MESSAGE_TIME_OUT: &str = "5000";

#[derive(Debug, Deserialize, Clone)]
pub struct ConnectorConfig {
    connector: HashMap<String, KafkaProperties>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KafkaProperties {
    broker: String,
    topic: String,
}

#[derive(Debug, Clone)]
pub struct ProducerMessage {
    kafka_props: KafkaProperties,
    json_generator: JsonGenerator,
}

/// Send message to kafka topic (data format dependencies config file)
impl ProducerMessage {
    pub fn new_json_producer(kafka_props: KafkaProperties, json_generator: JsonGenerator) -> Self {
        Self {
            kafka_props,
            json_generator,
        }
    }

    pub async fn send_message(&'static self) -> anyhow::Result<()> {
        let (tx, mut rx) = sync::mpsc::channel(DEFAULT_CHANNEL_BUFFER_SIZE);
        let (notify_tx, mut notify_rx) = sync::watch::channel(RunningState::Process);
        let topic_string = self.kafka_props.clone().topic;
        tokio::task::spawn(async move {
            let producer: &FutureProducer = &rdkafka::ClientConfig::new()
                .set(
                    "bootstrap.servers",
                    self.kafka_props.broker.clone().as_str(),
                )
                .set("message.timeout.ms", MESSAGE_TIME_OUT)
                .create()
                .expect("can't create kafka producer");
            println!("kafka producer create success.");
            let mut counter = 0_i32;
            loop {
                let msg: Option<String> = tokio::select! {
                   data = rx.recv() => {
                        Some(data.unwrap())
                   },
                   run_status = notify_rx.changed() => {
                        if run_status.is_ok() {
                            if let RunningState::Process = *notify_rx.borrow() {
                                continue;
                            } else {
                                println!("");
                                None
                            }
                        } else {
                            continue;
                        }
                   }
                };
                if let Some(data) = msg {
                    println!("receive data={}", data.clone());
                    let deliver_status = producer
                        .send::<Vec<u8>, _, _>(
                            FutureRecord::to(topic_string.as_str())
                                .payload(&data.as_bytes().to_vec()),
                            Duration::from_secs(0),
                        )
                        .await;
                    if deliver_status.is_err() {
                        println!(
                            "send message to kafka error topic={},msg={}",
                            topic_string, data
                        );
                        break;
                    } else {
                        if counter % 10 == 0 {
                            println!("{} {} messages sent successfully", topic_string, counter);
                        }
                        counter += 1;
                    }
                } else {
                    break;
                }
            }
        });
        let _msg_gen_rs = self.json_generator.batch_generate(tx, notify_tx).await;
        Ok(())
    }
}

pub fn load_kafka_props(conf_path: String) -> KafkaProperties {
    let connector_path = format!("{}/{}", conf_path, "connector.toml");
    let connector_file_content = read_file(connector_path.clone());
    let connector_rs: anyhow::Result<ConnectorConfig> =
        load_toml_config(connector_file_content.as_str());
    if connector_rs.is_err() {
        panic!("can't read file {}", connector_path);
    }
    connector_rs
        .unwrap()
        .connector
        .get("kafka")
        .unwrap()
        .clone()
}

pub fn new_generator(conf_path: String) -> JsonGenerator {
    let template_conf_path = format!("{}/{}", conf_path, "json-nonested.json");
    let rule_conf_path = format!("{}/{}", conf_path, "generator.toml");
    let json_template_rs = load_json_template(template_conf_path.clone());
    if json_template_rs.is_err() {
        panic!("can't read file {}", template_conf_path);
    }
    let rule_file_content = read_file(rule_conf_path.clone());
    let rule_config_rs: anyhow::Result<GeneratorRuleConfig> =
        load_toml_config(rule_file_content.as_str());
    if rule_config_rs.is_err() {
        panic!("can't read file {}", rule_conf_path);
    }
    let rule_config = rule_config_rs.unwrap().generator.get("jsonnonested").unwrap().clone();
    JsonGenerator::new(json_template_rs.unwrap(), rule_config)
}

pub fn new_producer_by_config(config_path: String) -> Box<ProducerMessage> {
    println!("current config path = {:?}", config_path.clone());
    let kafka_props = load_kafka_props(config_path.clone());
    let json_generator = new_generator(config_path);
    Box::new(ProducerMessage::new_json_producer(
        kafka_props,
        json_generator,
    ))
}

fn read_file(path: String) -> String {
    let read_rs = std::fs::read_to_string(path.clone());
    if let Ok(file_content) = read_rs {
        file_content
    } else {
        panic!("can't read file from path {}", path)
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use crate::new_producer_by_config;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn kafka_producer_test() {
        let current_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let parent = current_path.parent().unwrap().parent().unwrap();
        let dev_config_path = parent.as_os_str().to_str().unwrap().to_string();
        let producer = new_producer_by_config(format!("{}/configs", dev_config_path));
        let producer_static = Box::leak(producer);
        let rs = producer_static.send_message().await;
        println!("producer sens msg = {:?}", rs);
    }
}
