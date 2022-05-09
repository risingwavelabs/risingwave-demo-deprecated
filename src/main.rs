extern crate core;

use clap::Parser;
use workload_generator::{config::Config, kafka::Producer};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CmdArgs {
    /// Config file path.
    #[clap(short, long)]
    config: String,
}

#[tokio::main]
async fn main() {
    let args = CmdArgs::parse();

    let file = std::fs::read_to_string(&args.config).unwrap();
    let cfg: Config = serde_yaml::from_str(&file)
        .unwrap_or_else(|e| panic!("Failed to parse config file: {}\n{}", e, file));
    let producer = Producer::new(cfg);
    producer.run().await;
}
