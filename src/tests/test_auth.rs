use std::time::Duration;

use crate::auth::{authenticate_portal, start_portal};

#[tokio::test]
#[ignore]
async fn test_auth() {
    dotenvy::dotenv().ok();
    let path = std::env::var("IBCP_PATH").expect("IBCP_PATH must be set");
    let mut portal = start_portal(&path).await.unwrap();
    tokio::time::sleep(Duration::from_secs(4)).await;
    assert!(authenticate_portal().await.unwrap());
    portal.kill().await.unwrap();
}
