use config::Config;
use generator::Generator;
use governor::{Quota, RateLimiter};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use sink::Sink;

pub mod config;
pub mod generator;
pub mod rand;
pub mod sink;

/// Loop until all total records are sent or a failure occurs.
pub async fn run_loop(cfg: Config) {
    let generator = Generator::new(cfg.clone());
    let sink = Sink::new(cfg.clone()).await;

    let pb = ProgressBar::new(cfg.total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
            )
            .progress_chars("#>-"),
    );

    let qps = cfg.qps.unwrap_or(u32::max_value());
    let lim = RateLimiter::direct(Quota::per_second(qps.try_into().unwrap()));
    for _ in (0..cfg.total).progress_with(pb) {
        lim.until_ready().await;
        let msg = generator.generate();
        if let Err(e) = sink.send_record(&msg).await {
            println!("ERROR: failed to send message: {}\n{}", e, &msg);
            break;
        }
    }
}
