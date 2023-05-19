use serde_json::json;


use crate::models::{contract::{Contract}, exchanges::Exchange, tick_types::TickType};

pub enum SubscriptionType {
    QuoteData((Vec<TickType>, Contract)),
    HistoricalData,
    MarketDepth,
    Orders,
    Positions,
    Trades,
    ProfitLoss,
}

pub struct Subscription {
    pub sub_type: SubscriptionType,
    pub exchange: Option<Exchange>,
}
impl Subscription {
    pub fn new_smart_quote_data(contract:Contract) -> Self {
        let tick_types = vec![TickType::BidPrice, TickType::AskPrice,TickType::AskSize,TickType::BidSize,TickType::LastPrice];
        Self { sub_type: SubscriptionType::QuoteData((tick_types, contract)), exchange: None }
    }
    pub fn build(&self) -> String {
        match &self.sub_type {
            SubscriptionType::QuoteData((tick_types, contract)) => {
                let field_list = tick_types.iter().map(|t| serde_json::to_string(t).unwrap()).collect::<Vec<String>>();
                let contract_rep = match &self.exchange {
                    Some(exchange) => format!("{}@{exchange}",contract.conid),
                    None => contract.conid.to_string(),
                };
                let arg_json = json!({"fields":field_list});
                format!("smd+{contract_rep}+{}",arg_json)
            }
            SubscriptionType::HistoricalData => unimplemented!(),
            SubscriptionType::MarketDepth => unimplemented!(),
            SubscriptionType::Orders => unimplemented!(),
            SubscriptionType::Positions => unimplemented!(),
            SubscriptionType::Trades => {
                String::from("str+{}")
            },
            SubscriptionType::ProfitLoss => unimplemented!(),
        }
    }
}

