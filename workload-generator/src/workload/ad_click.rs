use super::Generator;
use crate::{
    config::{Config, FormatType},
    rand::{rand_long, rand_timestamp},
};
use maplit::hashmap;
use serde::Serialize;
use std::{collections::HashMap, time::Duration};

/*
ad_clicks => (
    user_id bigint,
    ad_id bigint,
    click_timestamp timestamp,
    impression_timestamp timestamp
)
*/

pub(crate) struct AdClickGen {
    cfg: Config,
}

impl AdClickGen {
    pub fn create(cfg: Config) -> Box<dyn Generator> {
        Box::new(Self { cfg })
    }
}

impl Generator for AdClickGen {
    fn generate_record(&mut self) -> HashMap<String, String> {
        let record = Record::new();
        let value = match self.cfg.format {
            FormatType::Json => serde_json::to_string(&record).unwrap(),
            FormatType::Sql(_) => todo!(),
        };
        hashmap! {
            "ad_clicks".to_string() => value
        }
    }
}

#[derive(Serialize)]
struct Record {
    user_id: i64,
    ad_id: i64,
    click_timestamp: String,
    impression_timestamp: String,
}

impl Record {
    fn new() -> Self {
        Self {
            user_id: rand_long(1, 100000),
            ad_id: rand_long(1, 10),
            click_timestamp: rand_timestamp(Some(Duration::from_secs(1))),
            impression_timestamp: rand_timestamp(None),
        }
    }
}
