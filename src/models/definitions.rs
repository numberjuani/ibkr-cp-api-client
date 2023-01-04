use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize,Default)]
pub enum AssetClass {
    #[serde(rename = "BOND")]
    Bond,
    #[serde(rename = "CFD")]
    Cfd,
    #[serde(rename = "FUT")]
    Future,
    #[serde(rename = "IND")]
    Index,
    #[serde(rename = "OPT")]
    Option,
    #[serde(rename = "STK")]
    Stock,
    #[serde(rename = "FOP")]
    FuturesOptions,
    #[serde(rename = "FUND")]
    MutualFund,
    #[serde(rename = "CMDTY")]
    Commodity,
    #[default] Unknown
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptionRight {
    #[serde(rename = "C")]
    Call,
    #[serde(rename = "P")]
    Put
}