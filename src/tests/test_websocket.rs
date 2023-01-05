use crate::{client::IBClientPortal, websocket::requests::{Subscription, SubscriptionType}, models::{contract::Contract, tick_types::TickType}};

fn print_message(msg: String) {
    println!("msg: {}", msg);
}
//cargo test -- --nocapture
#[tokio::test]
#[ignore]
async fn test_websocket() {
    let mut client = IBClientPortal::from_env();
    client = client.get_session_id().await.unwrap();
    let subs = vec![Subscription::new_smart_quote_data(Contract::from_con_id(265598))];
    client.connect_to_websocket(subs,print_message).await.unwrap();
}

#[tokio::test]
async fn test_subscription_builder() {
    let sub_type = SubscriptionType::QuoteData((vec![TickType::BidPrice, TickType::AskPrice],Contract::from_con_id(265598)));
    let subscription = Subscription { sub_type, exchange: None };
    let sub_str = subscription.build();
    assert_eq!(sub_str, "smd+265598+{\"fields\":[\"84\",\"86\"]}");
}

#[tokio::test]
async fn test_smart_quote() {
    let subscription = Subscription::new_smart_quote_data(Contract::from_con_id(265598));
    let sub_str = subscription.build();
    assert_eq!(sub_str, "smd+265598+{\"fields\":[\"84\",\"86\",\"85\",\"88\",\"31\"]}");
}