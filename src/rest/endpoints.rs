use std::collections::HashMap;

use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde_json::{json, Value};

use crate::client::IBClientPortal;
use crate::models::account_ledger::AccountLedger;
use crate::models::contract::SecurityDefinitions;
use crate::models::contract_detail::ContractDetail;
use crate::models::definitions::AssetClass;
use crate::models::futures_contract::FuturesContracts;
use crate::models::market_data_history::MarketDataHistory;
use crate::models::order_ticket::OrderTicket;
use crate::models::positions::Position;
use crate::models::stock_contracts::StockContracts;
use crate::models::tickle::AuthStatus;
use crate::models::tickle::Tickle;
//https://www.interactivebrokers.com/api/doc.html

pub async fn process_response<T: for<'a> serde::Deserialize<'a>>(
    response: reqwest::Response,
) -> Result<T, reqwest::Error> {
    response.json().await
}

impl IBClientPortal {
    /// Returns the url for the given path
    fn get_url(&self, path: &str) -> String {
        let base = if self.listen_ssl { "https" } else { "http" };
        format!("{base}://localhost:{}/v1/api{path}", self.port)
    }
    /// Current Authentication status to the Brokerage system.
    /// Market Data and Trading is not possible if not authenticated, e.g. authenticated shows false
    pub async fn check_auth_status(&self) -> Result<AuthStatus, reqwest::Error> {
        let response = self
            .client
            .post(self.get_url("/iserver/auth/status"))
            .header(
                reqwest::header::CONTENT_LENGTH,
                reqwest::header::HeaderValue::from_static("0"),
            )
            .body("")
            .send()
            .await?;
        response.json().await
    }
    /// If the gateway has not received any requests for several minutes an open session will automatically timeout.
    /// The tickle endpoint pings the server to prevent the session from ending.
    pub async fn tickle(&self) -> Result<Tickle, reqwest::Error> {
        let response = self
            .client
            .post(self.get_url("/tickle"))
            .header(
                reqwest::header::CONTENT_LENGTH,
                reqwest::header::HeaderValue::from_static("0"),
            )
            .body("")
            .send()
            .await?;
        response.json().await
    }
    /// Returns a list of positions for the given account.
    /// The endpoint supports paging, page's default size is 30 positions.
    /// `/portfolio/accounts` or `/portfolio/subaccounts` must be called prior to this endpoint.
    pub async fn get_positions(&self, page: i32) -> Result<Vec<Position>, reqwest::Error> {
        let path = format!("/portfolio/{}/positions/{}", self.account, page);
        let response = self.client.get(self.get_url(&path)).body("").send().await?;
        println!("{:#?}", response.status());
        process_response(response).await
    }
    ///Convenience method to call tickle and get the session id. It is necessary to auth the websocket connection.
    pub async fn get_session_id(&mut self) -> Result<(), reqwest::Error> {
        let response = self.tickle().await?;
        self.session_id = Some(response.session);
        Ok(())
    }
    ///Returns a list of security definitions for the given conids
    pub async fn get_security_definition_by_contract_id(
        &self,
        contract_ids: Vec<i64>,
    ) -> Result<SecurityDefinitions, reqwest::Error> {
        let path = "/trsrv/secdef";
        let payload = json!({
            "conids" : contract_ids,
        });
        let request = self.client.post(self.get_url(path));
        let response = request.body(payload.to_string()).send().await?;
        process_response(response).await
    }
    ///Returns a list of non-expired future contracts for given symbol(s)
    pub async fn get_futures_by_symbol(
        &self,
        symbols: Vec<&str>,
    ) -> Result<FuturesContracts, reqwest::Error> {
        let path = "/trsrv/futures";
        let request = self
            .client
            .get(self.get_url(path))
            .query(&[("symbols", symbols.join(","))]);
        let response = request.send().await?;
        process_response(response).await
    }
    ///Returns an object contains all stock contracts for given symbol(s)
    pub async fn get_stocks_by_symbol(
        &self,
        symbols: Vec<&str>,
    ) -> Result<StockContracts, reqwest::Error> {
        let path = "/trsrv/stocks";
        let request = self
            .client
            .get(self.get_url(path))
            .query(&[("symbols", symbols.join(","))]);
        let response = request.send().await?;
        process_response(response).await
    }
    ///Search by underlying symbol or company name. Relays back what derivative contract(s) it has. This endpoint must be called before using /secdef/info.
    /// If company name is specified will only receive limited response: conid, companyName, companyHeader and symbol.
    pub async fn search_for_security(
        &self,
        symbol_or_name: &str,
        is_name: bool,
        sec_type: AssetClass,
    ) -> Result<Value, reqwest::Error> {
        let path = "/iserver/secdef/search";
        let body = json!( {
            "symbol": symbol_or_name,
            "name": is_name,
            "secType": sec_type,
        });
        let request = self.client.post(self.get_url(path)).body(body.to_string());
        let response = request.send().await?;
        process_response(response).await
    }
    ///Provides Contract Details of Futures, Options, Warrants, Cash and CFDs based on conid. To get the strike price for Options/Warrants use "/iserver/secdef/strikes" endpoint.
    /// Must call /secdef/search for the underlying contract first.
    pub async fn get_options(
        &self,
        underlying_con_id: i64,
        sectype: AssetClass,
        month: Option<String>,
        exchange: Option<String>,
        strike: Option<Decimal>,
    ) -> Result<Value, reqwest::Error> {
        let path = "/iserver/secdef/info";
        let mut query = vec![
            ("conid", underlying_con_id.to_string()),
            ("sectype", sectype.to_string()),
        ];
        if let Some(month) = month {
            query.push(("month", month));
        }
        if let Some(exchange) = exchange {
            query.push(("exchange", exchange));
        }
        if let Some(strike) = strike {
            query.push(("strike", strike.to_string()));
        }
        let response = self
            .client
            .get(self.get_url(path))
            .query(&query)
            .send()
            .await?;
        response.json().await
    }
    ///Logs the user out of the gateway session. Any further activity requires re-authentication.
    pub async fn logout(&self) -> Result<Value, reqwest::Error> {
        let response = self
            .client
            .post(self.get_url("/logout"))
            .header(
                reqwest::header::CONTENT_LENGTH,
                reqwest::header::HeaderValue::from_static("0"),
            )
            .body("")
            .send()
            .await?;
        response.json().await
    }

    pub async fn get_account_ledger(
        &self,
    ) -> Result<HashMap<String, AccountLedger>, reqwest::Error> {
        let path = format!("/portfolio/{}/ledger", self.account);
        let response = self.client.get(self.get_url(&path)).body("").send().await?;
        process_response(response).await
    }

    pub async fn place_order(&self, orders: Vec<OrderTicket>) -> Result<Value, reqwest::Error> {
        let path = format!("/iserver/account/{}/order", self.account);
        let payload = json!({"orders":orders});
        let request = self.client.post(self.get_url(&path));
        let response = request.body(payload.to_string()).send().await?;
        process_response(response).await
    }

    /// Get contracts details. Many fields are optional and do not match the api documentation.
    pub async fn get_contract_detail(&self, conid: i64) -> Result<ContractDetail, reqwest::Error> {
        let path = format!("/iserver/contract/{}/info", conid);
        let response = self.client.get(self.get_url(&path)).body("").send().await?;
        response.json().await
    }

    /// Get market data history
    /// tradingDayDuration is not always present if period is less than 1 day
    /// exchange is optional and will be set to "" if not provided
    /// startTime is optional and not documented
    /// to retrieve 1min bars the startTime  should be 2 minutes after the timestamp expected in the last bar of the response
    pub async fn get_market_data_history(
        &self,
        conid: i64,
        exchange: Option<&str>,
        period: &str,
        bar: &str,
        start_time: Option<NaiveDateTime>,
    ) -> Result<MarketDataHistory, reqwest::Error> {
        let path = "/iserver/marketdata/history";
        let start_time_str = match start_time {
            Some(start_time) => start_time.format("%Y%m%d-%H:%M:%S").to_string(),
            None => "".to_string(),
        };

        let request = self
            .client
            .get(self.get_url(path))
            .query(&[("conid", conid)])
            .query(&[("period", period)])
            .query(&[("bar", bar)])
            .query(&[("exchange", exchange.unwrap_or(""))])
            .query(&[("startTime", start_time_str)]);
        let response = request.send().await?;
        response.json().await
    }
}
