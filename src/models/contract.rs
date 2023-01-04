use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::definitions::{AssetClass, OptionRight};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityDefinitions {
    pub secdef: Vec<Contract>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize,Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Contract {
    pub all_exchanges: String,
    pub asset_class: AssetClass,
    pub chinese_name: String,
    pub conid: i64,
    pub country_code: String,
    pub currency: String,
    pub display_rule: DisplayRule,
    pub expiry: Option<String>,
    pub full_name: String,
    pub group: String,
    pub has_options: bool,
    pub increment_rules: Vec<IncrementRule>,
    pub is_event_contract: bool,
    #[serde(rename = "isUS")]
    pub is_us: Option<bool>,
    pub last_trading_day: Option<String>,
    pub listing_exchange: String,
    pub multiplier: f64,
    pub name: Option<String>,
    pub page_size: Option<i64>,
    pub put_or_call: Option<OptionRight>,
    pub sector: String,
    pub sector_group: Option<String>,
    pub strike: String,
    pub ticker: String,
    pub time: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub und_conid: i64,
    pub cross_currency: Option<bool>,
    pub und_comp: Option<Value>,
    pub und_sym: Option<String>,
}
impl Contract {
    pub fn from_con_id(con_id: i64) -> Self {
        Self {
            conid: con_id,
            ..Default::default()
        }
    }
}
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
