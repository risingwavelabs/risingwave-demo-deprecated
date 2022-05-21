extern crate core;

use clap::Parser;
use core::panic;
use validator::Validate;
use workload_generator::{config::Config, run_loop, workload::Workload};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CmdArgs {
    /// Config file path.
    #[clap(short, long)]
    config: String,

    /// Which workload model to run.
    #[clap(arg_enum)]
    workload: Workload,
}

#[tokio::main]
async fn main() {
    env_logger::builder().init();

    let args = CmdArgs::parse();

    let file = std::fs::read_to_string(&args.config)
        .unwrap_or_else(|e| panic!("Failed to read config file ({}): {}", args.config, e));
    let cfg: Config = serde_yaml::from_str(&file)
        .unwrap_or_else(|e| panic!("Failed to parse config file: {}\n{}", e, file));
    cfg.validate().unwrap();
    run_loop(args.workload, cfg).await;
}
