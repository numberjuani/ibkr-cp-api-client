use chrono::naive::serde::ts_milliseconds;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bar {
    #[serde(with = "ts_milliseconds")]
    #[serde(alias = "t")]
    pub timestamp: NaiveDateTime,
    #[serde(alias = "o")]
    pub open: rust_decimal::Decimal,
    #[serde(alias = "c")]
    pub close: rust_decimal::Decimal,
    #[serde(alias = "h")]
    pub high: rust_decimal::Decimal,
    #[serde(alias = "l")]
    pub low: rust_decimal::Decimal,
    #[serde(alias = "v")]
    pub volume: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketDataHistory {
    pub server_id: String,
    pub symbol: String,
    pub text: String,
    pub price_factor: i64,
    #[serde(with = "parse_datetime")]
    pub start_time: NaiveDateTime,
    pub high: String,
    pub low: String,
    pub time_period: String,
    pub bar_length: u32,
    pub md_availability: String,
    pub mkt_data_delay: i64,
    pub outside_rth: bool,
    pub trading_day_duration: Option<i64>,
    pub volume_factor: i64,
    pub price_display_rule: i64,
    pub price_display_value: String,
    pub negative_capable: bool,
    pub message_version: i64,
    pub data: Vec<Bar>,
    pub points: u32,
    pub travel_time: u32,
}

mod parse_datetime {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format("%Y%m%d-%H:%M:%S"));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_string();
        NaiveDateTime::parse_from_str(&s, "%Y%m%d-%H:%M:%S").map_err(serde::de::Error::custom)
    }
}
