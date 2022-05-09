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

    let file = std::fs::File::open(&args.config).unwrap();
    let cfg: Config = serde_yaml::from_reader(file).unwrap();
    let producer = Producer::new(cfg);
    producer.run().await;
}
