use chrono::serde::ts_seconds;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize,Default)]
#[serde(default)]
pub struct AccountLedger {
    pub commoditymarketvalue: Decimal,
    pub futuremarketvalue: Decimal,
    pub settledcash: Decimal,
    pub exchangerate: Decimal,
    pub sessionid: Decimal,
    pub cashbalance: Decimal,
    pub corporatebondsmarketvalue: Decimal,
    pub warrantsmarketvalue: Decimal,
    pub netliquidationvalue: Decimal,
    pub interest: Decimal,
    pub unrealizedpnl: Decimal,
    pub stockmarketvalue: Decimal,
    pub moneyfunds: Decimal,
    pub currency: String,
    pub realizedpnl: Decimal,
    pub funds: Decimal,
    pub acctcode: String,
    pub issueroptionsmarketvalue: Decimal,
    pub key: String,
    #[serde(with = "ts_seconds")]
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub severity: Decimal,
}