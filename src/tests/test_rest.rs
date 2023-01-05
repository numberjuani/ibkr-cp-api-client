use crate::{client::IBClientPortal, models::{definitions::AssetClass, exchanges::Exchange}};



#[tokio::test]
#[ignore]
async fn test_tickle() {
    let ib_cp_api_client = IBClientPortal::from_env();
    let tickle = ib_cp_api_client.tickle().await;
    println!("{:#?}", tickle);
    assert!(tickle.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_positions() {
    let ib_cp_api_client = IBClientPortal::from_env();
    let positions = ib_cp_api_client.get_positions(0).await;
    println!("{:#?}", positions);
    assert!(positions.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_sec_def_by_con_id() {
    let ib_cp_api_client = IBClientPortal::from_env();
    let contracts = ib_cp_api_client.get_security_definition_by_contract_id(vec![265598,416904,495512572]).await;
    println!("{:#?}",contracts);
    assert!(contracts.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_futures_by_symbol() {
    let ib_cp_api_client = IBClientPortal::from_env();
    let contracts = ib_cp_api_client.get_futures_by_symbol(vec!["ES","NQ"]).await;
    println!("{:#?}",contracts);
    assert!(contracts.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_stocks_by_symbol() {
    let ib_cp_api_client = IBClientPortal::from_env();
    let contracts = ib_cp_api_client.get_stocks_by_symbol(vec!["AAPL","GOOG","QQQ"]).await;
    println!("{:#?}",contracts);
    assert!(contracts.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_search_for_security() {
    let ib_cp_api_client = IBClientPortal::from_env();
    let apple = ib_cp_api_client.search_for_security("AAPL", false, AssetClass::Option).await.unwrap();
    println!("{}",serde_json::to_string_pretty(&apple).unwrap());
}

#[tokio::test]
async fn test_exchange() {
    let exch_str = "NYSE";
    let exch = Exchange::from_str(exch_str);
    assert_eq!(exch, Exchange::NYSE);
}