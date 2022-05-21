use chrono::{DateTime, Utc};
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

// TODO: support for loading from config file.
const DEFAULT_RANDOM_LEN: i32 = 6;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    StringZh,
    String,
    Name,
    Enum(Vec<String>),
    Long { start: i64, stop: i64 },
    Float { start: f64, stop: f64 },
    Timestamp(Option<Duration>),
}

pub fn new_sql_field(data_type: DataType) -> String {
    use crate::rand::*;
    match data_type {
        DataType::StringZh => format!("'{}'", rand_string_zh()),
        DataType::String => format!("'{}'", rand_string()),
        DataType::Enum(variants) => format!("'{}'", rand_enum(&variants)),
        DataType::Long { start, stop } => rand_long(start, stop).to_string(),
        DataType::Float { start, stop } => rand_float(start, stop).to_string(),
        DataType::Timestamp(random_delay) => format!("'{}'", rand_timestamp(random_delay)),
        DataType::Name => format!("'{}'", rand_name()),
    }
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

pub fn rand_timestamp(random_delay: Option<Duration>) -> String {
    let now = SystemTime::now();
    let time = match random_delay {
        Some(t) => {
            let delay_ms = rand::thread_rng().gen_range(0_u64..(t.as_secs() * 1000));
            now + Duration::from_millis(delay_ms)
        }
        None => now,
    };
    DateTime::<Utc>::from(time)
        .format("%Y-%m-%d %H:%M:%S%.f")
        .to_string()
}

pub fn rand_long(start: i64, stop: i64) -> i64 {
    rand::thread_rng().gen_range(start..stop)
}

pub fn rand_float(start: f64, stop: f64) -> f64 {
    rand::thread_rng().gen_range(start..stop)
}

pub fn rand_percent() -> f64 {
    rand_float(0_f64, 100_f64)
}

pub fn rand_name() -> String {
    let mut rng = rand::thread_rng();
    petname::Petnames::default().generate(&mut rng, 2, " ")
}

#[cfg(test)]
mod tests {
    use super::rand_timestamp;
    use chrono::NaiveDateTime;
    use std::time::Duration;

    #[test]
    fn test_timestmap() {
        assert_eq!(
            "2022-05-09 13:26:07.396503".len(),
            rand_timestamp(None).len()
        );

        fn parse_timestamp(s: &str) -> NaiveDateTime {
            NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S%.f").unwrap()
        }

        let ts1 = parse_timestamp(&rand_timestamp(Some(Duration::from_secs(1))));
        let ts2 = parse_timestamp(&rand_timestamp(None));

        assert!(ts1 > ts2);
        assert!((ts1 - ts2) <= chrono::Duration::seconds(1));
    }
}
