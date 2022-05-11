use chrono::{DateTime, Utc};
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

use crate::config::TimestampConfig;

// TODO: support for loading from config file.
const RAN_F64_MAX: f64 = 50000_f64;
const RAN_I64_MAX: i64 = 10000000_i64;
const RAN_I32_MAX: i32 = 40000_i32;
const DEFAULT_RANDOM_LEN: i32 = 6;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    StringZh,
    String,
    Enum,
    Long,
    Int,
    Float,
    Timestamp,
}

pub fn rand_enum(variants: &[String]) -> String {
    let rand_seed: usize = rand::thread_rng().gen_range(1..200);
    let rand_idx = rand_seed % variants.len();
    variants[rand_idx].clone()
}

pub fn rand_string_zh() -> String {
    (0..DEFAULT_RANDOM_LEN)
        .map(|_| {
            let rand_u32 = rand::thread_rng().gen_range(0..20902_u32) + 19968_u32;
            char::from_u32(rand_u32).unwrap()
        })
        .collect()
}

pub fn rand_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(DEFAULT_RANDOM_LEN as usize)
        .map(char::from)
        .collect()
}

pub fn rand_timestamp(time_cfg: &Option<TimestampConfig>) -> String {
    let now = SystemTime::now();
    let time = match time_cfg {
        Some(t) => {
            let delay_ms = rand::thread_rng().gen_range(0_u64..(t.random_delay.as_secs() * 1000));
            now + Duration::from_millis(delay_ms)
        }
        None => now,
    };
    DateTime::<Utc>::from(time)
        .format("%Y-%m-%d %H:%M:%S%.f")
        .to_string()
}

pub fn rand_long() -> i64 {
    rand::thread_rng().gen_range(1_i64..RAN_I64_MAX)
}

pub fn rand_int() -> i32 {
    rand::thread_rng().gen_range(1_i32..RAN_I32_MAX)
}

pub fn rand_float() -> f64 {
    rand::thread_rng().gen_range(1_f64..RAN_F64_MAX)
}

#[cfg(test)]
mod tests {
    use super::rand_timestamp;
    use crate::config::TimestampConfig;
    use chrono::NaiveDateTime;
    use std::time::Duration;

    #[test]
    fn test_timestmap() {
        assert_eq!(
            "2022-05-09 13:26:07.396503".len(),
            rand_timestamp(&None).len()
        );

        fn parse_timestamp(s: &str) -> NaiveDateTime {
            NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S%.f").unwrap()
        }

        let ts1 = parse_timestamp(&rand_timestamp(&Some(TimestampConfig {
            random_delay: Duration::from_secs(1),
        })));
        let ts2 = parse_timestamp(&rand_timestamp(&None));

        assert!(ts1 > ts2);
        assert!((ts1 - ts2) <= chrono::Duration::seconds(1));
    }
}
