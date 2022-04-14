use rand::distributions::Alphanumeric;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};
use strum_macros::EnumString;

// TODO: support for loading from config file.
const PLATFORM_ENUM_ARRAY: &[&str] = &["android", "ios"];
const RAN_F64_MAX: f64 = 50000_f64;
const RAN_I64_MAX: i64 = 10000000_i64;
const RAN_I32_MAX: i32 = 40000_i32;
const DEFAULT_RANDOM_LEN: i32 = 6;

#[derive(Debug, Clone, PartialEq, EnumString)]
pub enum JsonDataType {
    #[strum(ascii_case_insensitive)]
    StringZh,
    #[strum(ascii_case_insensitive)]
    String,
    #[strum(ascii_case_insensitive)]
    Enum,
    #[strum(ascii_case_insensitive)]
    Long,
    #[strum(ascii_case_insensitive)]
    Int,
    #[strum(ascii_case_insensitive)]
    Float,
    #[strum(ascii_case_insensitive)]
    Timestamp,
}

pub trait Value<T> {
    fn get_value(&self, json_type: Option<JsonDataType>) -> T;
}

pub(crate) struct RandomStringValue;

impl Value<String> for RandomStringValue {
    fn get_value(&self, json_type: Option<JsonDataType>) -> String {
        match json_type.unwrap() {
            JsonDataType::StringZh => (0..DEFAULT_RANDOM_LEN)
                .map(|_| {
                    let rand_u32 = rand::thread_rng().gen_range(0..20902_u32) + 19968_u32;
                    char::from_u32(rand_u32).unwrap()
                })
                .collect(),
            JsonDataType::String => rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(DEFAULT_RANDOM_LEN as usize)
                .map(char::from)
                .collect(),
            JsonDataType::Enum => {
                let rand_seed: usize = rand::thread_rng().gen_range(1..200);
                let rand_idx = rand_seed % PLATFORM_ENUM_ARRAY.len();
                PLATFORM_ENUM_ARRAY[rand_idx].to_string()
            }
            _ => {
                unreachable!()
            }
        }
    }
}

pub(crate) struct LongValue {}

impl Value<i64> for LongValue {
    fn get_value(&self, json_type: Option<JsonDataType>) -> i64 {
        match json_type.unwrap() {
            JsonDataType::Timestamp => SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
            JsonDataType::Long => rand::thread_rng().gen_range(1_i64..RAN_I64_MAX),
            _ => {
                unreachable!()
            }
        }
    }
}

pub(crate) struct IntValue;

impl Value<i32> for IntValue {
    fn get_value(&self, _json_type: Option<JsonDataType>) -> i32 {
        rand::thread_rng().gen_range(1_i32..RAN_I32_MAX)
    }
}

pub(crate) struct FloatValue;

impl Value<f64> for FloatValue {
    fn get_value(&self, _json_type: Option<JsonDataType>) -> f64 {
        rand::thread_rng().gen_range(1_f64..RAN_F64_MAX)
    }
}
