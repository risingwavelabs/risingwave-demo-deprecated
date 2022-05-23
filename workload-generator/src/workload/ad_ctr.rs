use super::Generator;
use crate::{
    config::{Config, FormatType},
    rand::{rand_long, rand_percent, rand_timestamp},
};
use maplit::hashmap;
use serde::Serialize;
use std::{collections::HashMap, time::Duration};

/*
ad_click => (
    bid_id bigint,
    click_timestamp timestamp,
)
ad_impression => (
    bid_id bigint,
    ad_id bigint,
    impression_timestamp timestamp
)
*/

pub(crate) struct AdCtrGen {
    cfg: Config,

    /// CTR of each ad.
    ad_ctr: HashMap<i64, f64>,
}

impl AdCtrGen {
    pub fn create(cfg: Config) -> Box<dyn Generator> {
        Box::new(Self {
            cfg,
            ad_ctr: HashMap::new(),
        })
    }

    fn fmt_click(&self, v: AdClick) -> String {
        match self.cfg.format {
            FormatType::Json => serde_json::to_string(&v).unwrap(),
            FormatType::Sql(_) => todo!(),
        }
    }

    fn fmt_impression(&self, v: AdImpression) -> String {
        match self.cfg.format {
            FormatType::Json => serde_json::to_string(&v).unwrap(),
            FormatType::Sql(_) => todo!(),
        }
    }

    fn ctr(&mut self, ad_id: i64) -> f64 {
        let v = self.ad_ctr.entry(ad_id).or_insert_with(rand_percent);
        *v
    }

    fn has_click(&mut self, ad_id: i64) -> bool {
        let rate = self.ctr(ad_id);
        rand_percent() < rate
    }
}

impl Generator for AdCtrGen {
    fn generate_record(&mut self) -> HashMap<String, String> {
        let bid_id = rand_long(100000, i64::MAX);
        let ad_id = rand_long(1, 10);

        let mut v = hashmap! {
            "ad_impression".to_string() => self.fmt_impression(AdImpression {
                bid_id,
                ad_id,
                impression_timestamp: rand_timestamp(None),
            })
        };
        if self.has_click(ad_id) {
            v.insert(
                "ad_click".to_string(),
                self.fmt_click(AdClick {
                    bid_id,
                    click_timestamp: rand_timestamp(Some(Duration::from_secs(1))),
                }),
            );
        }
        v
    }
}

#[derive(Serialize)]
struct AdClick {
    bid_id: i64,
    click_timestamp: String,
}

#[derive(Serialize)]
struct AdImpression {
    bid_id: i64,
    ad_id: i64,
    impression_timestamp: String,
}
