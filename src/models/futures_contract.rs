use std::collections::HashMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

pub type FuturesContracts = HashMap<String, Vec<FuturesContract>>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesContract {
    pub conid: i64,
    #[serde(with = "parse_date")]
    pub expiration_date: NaiveDate,
    #[serde(with = "parse_date")]
    pub long_futures_cut_off: NaiveDate,
    #[serde(with = "parse_date")]
    pub ltd: NaiveDate,
    #[serde(with = "parse_date")]
    pub short_futures_cut_off: NaiveDate,
    pub symbol: String,
    pub underlying_conid: i64,
}

mod parse_date {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format("%Y%m%d"));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = i64::deserialize(deserializer)?.to_string();
        NaiveDate::parse_from_str(&s, "%Y%m%d").map_err(serde::de::Error::custom)
    }
}
