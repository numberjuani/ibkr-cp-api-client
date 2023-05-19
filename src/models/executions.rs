
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, serde::ts_milliseconds};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Execution {
    #[serde(rename = "execution_id")]
    pub execution_id: String,
    pub symbol: String,
    #[serde(rename = "supports_tax_opt")]
    pub supports_tax_opt: String,
    pub side: String,
    #[serde(rename = "order_description")]
    pub order_description: String,
    #[serde(rename = "trade_time")]
    pub trade_time: String,
    #[serde(rename = "trade_time_r")]
    #[serde(with = "ts_milliseconds")]
    pub trade_time_r: DateTime<Utc>,
    pub size: i64,
    pub price: String,
    #[serde(rename = "order_ref")]
    pub order_ref: String,
    pub submitter: String,
    pub exchange: String,
    pub commission: String,
    #[serde(rename = "net_amount")]
    pub net_amount: Decimal,
    pub account: String,
    pub account_code: String,
    #[serde(rename = "company_name")]
    pub company_name: String,
    #[serde(rename = "contract_description_1")]
    pub contract_description_1: String,
    #[serde(rename = "sec_type")]
    pub sec_type: String,
    #[serde(rename = "listing_exchange")]
    pub listing_exchange: String,
    pub conid: String,
    pub conidex: String,
    #[serde(rename = "clearing_id")]
    pub clearing_id: String,
    #[serde(rename = "clearing_name")]
    pub clearing_name: String,
    #[serde(rename = "liquidation_trade")]
    pub liquidation_trade: String,
}
