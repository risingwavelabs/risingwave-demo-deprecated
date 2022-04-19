extern crate core;

pub mod kafka;

use crate::kafka::producer::new_producer_by_config;
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CmdArgs {
    #[clap(short, long)]
    conf: String,
}

#[tokio::main]
async fn main() {
    let cli = CmdArgs::parse();
    let config_path: String = cli.conf;
    let producer = new_producer_by_config(config_path);
    let producer_static = Box::leak(producer);
    let send_msg_rs = producer_static.send_message().await;
    println!("KafkaProducer send message complete. rs={:?}", send_msg_rs);
}
