use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::models::contract::Contract;

use super::definitions::AssetClass;

pub type StockContracts = HashMap<String,Vec<StockContractInfo>>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct StockContractInfo {
    pub asset_class: AssetClass,
    pub chinese_name: Option<String>,
    pub contracts: Vec<Contract>,
    pub name: String,
}

