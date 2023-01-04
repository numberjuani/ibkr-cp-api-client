use crate::client::IBClientPortal;



#[tokio::test]
async fn test_tickle() {
    let ib_cp_api_client = IBClientPortal::new(3000, false, "");
    let tickle = ib_cp_api_client.tickle().await;
    println!("{:#?}", tickle);
    assert!(tickle.is_ok());
}

#[tokio::test]
async fn test_positions() {
    let ib_cp_api_client = IBClientPortal::new(3000, false, "needs-a-real-account-number");
    let positions = ib_cp_api_client.get_positions(0).await;
    println!("{:#?}", positions);
    assert!(positions.is_ok());
}

#[tokio::test]
async fn test_sec_def_by_con_id() {
    let ib_cp_api_client = IBClientPortal::new(3000, false, "");
    let contracts = ib_cp_api_client.get_security_definition_by_contract_id(vec![265598,416904,603761154]).await;
    println!("{:#?}",contracts);
    assert!(contracts.is_ok());
}

#[tokio::test]
async fn test_futures_by_symbol() {
    let ib_cp_api_client = IBClientPortal::new(3000, false, "");
    let contracts = ib_cp_api_client.get_futures_by_symbol(vec!["ES","NQ"]).await;
    println!("{:#?}",contracts);
    assert!(contracts.is_ok());
}

#[tokio::test]
async fn test_stocks_by_symbol() {
    let ib_cp_api_client = IBClientPortal::new(3000, false, "");
    let contracts = ib_cp_api_client.get_stocks_by_symbol(vec!["AAPL","GOOG","QQQ"]).await;
    println!("{:#?}",contracts);
    assert!(contracts.is_ok());
}