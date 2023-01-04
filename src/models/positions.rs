use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use super::definitions::AssetClass;
use super::definitions::OptionRight;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub acct_id: String,
    pub all_exchanges: String,
    pub asset_class: AssetClass,
    pub avg_cost: f64,
    pub avg_price: f64,
    pub base_avg_cost: f64,
    pub base_avg_price: f64,
    pub base_mkt_price: f64,
    pub base_mkt_value: f64,
    pub base_realized_pnl: f64,
    pub base_unrealized_pnl: f64,
    #[serde(skip)]
    pub chinese_name: String,
    pub con_exch_map: Vec<Value>,
    pub conid: i64,
    pub contract_desc: String,
    pub country_code: String,
    pub cross_currency: Option<bool>,
    pub currency: String,
    #[serde(skip)]
    pub display_rule: DisplayRule,
    pub exchs: Value,
    pub exercise_style: Value,
    pub expiry: Option<String>,
    pub full_name: String,
    pub group: String,
    pub has_options: bool,
    #[serde(skip)]
    pub increment_rules: Vec<IncrementRule>,
    pub is_event_contract: bool,
    #[serde(rename = "isUS")]
    pub is_us: bool,
    pub last_trading_day: Option<String>,
    pub listing_exchange: String,
    pub mkt_price: f64,
    pub mkt_value: f64,
    pub model: String,
    pub multiplier: f64,
    pub name: Option<String>,
    pub page_size: i64,
    pub position: f64,
    pub put_or_call: Option<OptionRight>,
    pub realized_pnl: f64,
    pub sector: String,
    pub sector_group: Option<String>,
    pub strike: String,
    pub ticker: String,
    pub time: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub und_comp: Option<Value>,
    pub und_conid: i64,
    pub und_sym: Option<String>,
    pub unrealized_pnl: f64,
}
//"20230103"
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayRule {
    pub display_rule_step: Vec<DisplayRuleStep>,
    pub magnification: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayRuleStep {
    pub decimal_digits: i64,
    pub lower_edge: f64,
    pub whole_digits: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncrementRule {
    pub increment: f64,
    pub lower_edge: f64,
}


