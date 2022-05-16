extern crate core;

use clap::Parser;
use validator::Validate;
use workload_generator::{config::Config, run_loop};

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
    cfg.validate().unwrap();
    run_loop(cfg).await;
}
