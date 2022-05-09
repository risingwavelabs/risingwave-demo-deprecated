use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

// TODO: support for loading from config file.
const RAN_F64_MAX: f64 = 50000_f64;
const RAN_I64_MAX: i64 = 10000000_i64;
const RAN_I32_MAX: i32 = 40000_i32;
const DEFAULT_RANDOM_LEN: i32 = 6;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
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

pub fn rand_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
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
