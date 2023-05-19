use std::time::Duration;

use crate::utils::{start_portal, authenticate_portal};



#[tokio::test]
#[ignore]
async fn test_auth() {
    dotenv::dotenv().ok();
    let path = std::env::var("IBCP_PATH").expect("IB_CONF must be set");
    let _ = start_portal(&path).await.unwrap();
    tokio::time::sleep(Duration::from_secs(4)).await;
    assert!(authenticate_portal().await.unwrap())
}