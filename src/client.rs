use std::env;

/// This is the base class of the library. It is used to communicate with the IB CP Gateway.
///
/// https://interactivebrokers.github.io/cpwebapi/endpoints
///
/// bin/run.sh root/conf.yaml
#[derive(Debug, Default)]
pub struct IBClientPortal {
    pub port: i32,
    pub listen_ssl: bool,
    pub account: String,
    pub client: reqwest::Client,
    pub session_id: Option<String>,
}
impl IBClientPortal {
    pub fn new(port: i32, listen_ssl: bool, account: &str) -> IBClientPortal {
        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        default_headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static("Console"),
        );
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .default_headers(default_headers)
            .build()
            .unwrap();
        IBClientPortal {
            port,
            client,
            listen_ssl,
            account: account.to_string(),
            session_id: None,
        }
    }
    ///Builds the client from env vars. Needs
    /// `IB_PORT`
    /// `IB_SSL`
    /// `IB_ACCOUNT`
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        let port = env::var("IB_PORT")
            .expect("IB_PORT must be set")
            .parse()
            .unwrap();
        let listen_ssl = env::var("IB_SSL").unwrap_or_default().parse().unwrap();
        let account = env::var("IB_ACCOUNT").expect("IB_ACCOUNT must be set");
        IBClientPortal::new(port, listen_ssl, &account)
    }
}
