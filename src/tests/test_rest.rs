use std::str::FromStr;

use crate::{
    client::IBClientPortal,
    models::{definitions::AssetClass, exchanges::Exchange},
};

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
    let contracts = ib_cp_api_client
        .get_security_definition_by_contract_id(vec![265598, 416904, 495512572])
        .await;
    println!("{:#?}", contracts);
    assert!(contracts.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_contract_detail() {
    let ib_cp_api_client = IBClientPortal::from_env();
    for conid in [265598, 416904, 495512572] {
        let contract_details = ib_cp_api_client.get_contract_detail(conid).await;
        println!("{:#?}", contract_details);
        assert!(contract_details.is_ok());
        assert_eq!(contract_details.unwrap().conid, conid);
    }
}

#[tokio::test]
#[ignore]
async fn test_futures_by_symbol() {
    let ib_cp_api_client = IBClientPortal::from_env();
    let contracts = ib_cp_api_client
        .get_futures_by_symbol(vec!["ES", "NQ"])
        .await;
    println!("{:#?}", contracts);
    assert!(contracts.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_stocks_by_symbol() {
    let ib_cp_api_client = IBClientPortal::from_env();
    let contracts = ib_cp_api_client
        .get_stocks_by_symbol(vec!["AAPL", "GOOG", "QQQ"])
        .await;
    println!("{:#?}", contracts);
    assert!(contracts.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_search_for_security() {
    let ib_cp_api_client = IBClientPortal::from_env();
    let apple = ib_cp_api_client
        .search_for_security("AAPL", false, AssetClass::Option)
        .await
        .unwrap();
    println!("{}", serde_json::to_string_pretty(&apple).unwrap());
}

#[tokio::test]
#[ignore]
async fn test_get_market_data_history() {
    let ib_cp_api_client = IBClientPortal::from_env();
    let history = ib_cp_api_client
        .get_market_data_history(416904, None, "1w", "1h", false, None)
        .await;
    assert!(history.is_ok());
    let history = history.unwrap();
    assert_eq!(history.time_period, "1w");
    assert_eq!(history.bar_length, 3600);

    // tradingDayDuration is not always present if period is less than 1 day
    let history = ib_cp_api_client
        .get_market_data_history(416904, Some(""), "12h", "1h", false, None)
        .await;
    assert!(history.is_ok());

    // use optional startTime parameter which is not documented
    // data.len() is 13 but points is 12
    let history = ib_cp_api_client
        .get_market_data_history(
            416904,
            None,
            "12h",
            "1h",
            false,
            Some(
                chrono::naive::NaiveDateTime::parse_from_str(
                    "2023-08-31T19:00:00Z",
                    "%Y-%m-%dT%H:%M:%SZ",
                )
                .unwrap(),
            ),
        )
        .await;
    assert!(history.is_ok());
    let history = history.unwrap();
    assert_eq!(history.data.len(), 13);
    assert_eq!(history.points, Some(12));
    assert_eq!(
        history.data.iter().last().unwrap().timestamp,
        chrono::naive::NaiveDateTime::parse_from_str("2023-08-31T17:00:00Z", "%Y-%m-%dT%H:%M:%SZ")
            .unwrap()
    );

    // nonexistent conid is an error
    let history = ib_cp_api_client
        .get_market_data_history(123, Some(""), "12h", "1h", false, None)
        .await;
    assert!(history.is_err());
}

#[tokio::test]
async fn test_exchange() {
    let exch_str = "NYSE";
    let exch = Exchange::from_str(exch_str).unwrap();
    assert_eq!(exch, Exchange::NYSE);
}

#[tokio::test]
#[ignore]
async fn test_logout() {
    let ib_cp_api_client = IBClientPortal::from_env();
    let logout = ib_cp_api_client.logout().await;
    println!("{:#?}", logout);
    assert!(logout.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_account_ledger() {
    let ib_cp_api_client = IBClientPortal::from_env();
    let ledger = ib_cp_api_client.get_account_ledger().await;
    println!("{:#?}", ledger);
    assert!(ledger.is_ok());
}
