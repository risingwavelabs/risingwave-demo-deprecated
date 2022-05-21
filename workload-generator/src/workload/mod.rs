use crate::config::Config;
use std::collections::HashMap;

use self::{ad_click::AdClickGen, ad_ctr::AdCtrGen};

mod ad_click;
mod ad_ctr;

pub trait Generator {
    fn generate_record(&mut self) -> HashMap<String, String>;
}

#[derive(clap::ArgEnum, Clone, Copy)]
pub enum Workload {
    AdClick,
    AdCtr,
}

pub fn new_workload(workload: Workload, cfg: Config) -> Box<dyn Generator> {
    match workload {
        Workload::AdClick => AdClickGen::create(cfg),
        Workload::AdCtr => AdCtrGen::create(cfg),
    }
}
