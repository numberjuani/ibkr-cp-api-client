
use serde_json::{Value, json};

use crate::client::IBClientPortal;
use crate::models::contract::{SecurityDefinitions};
use crate::models::futures_contract::FuturesContracts;
use crate::models::positions::Position;
use crate::models::stock_contracts::{StockContracts};
//https://www.interactivebrokers.com/api/doc.html



pub async fn process_response<T: for<'a> serde::Deserialize<'a>>(response: reqwest::Response) -> Result<T, reqwest::Error> {
    Ok(response.json().await?)
}

impl IBClientPortal {
    fn get_url(&self, path: &str) -> String {
        let base = if self.listen_ssl { "https" } else { "http" };
        format!("{base}://localhost:{}/v1/api{path}", self.port)
    }
    pub async fn check_auth_status(&self) -> Result<Value, reqwest::Error> {
        let response = self.client.post(self.get_url("/iserver/auth/status")).body("").send().await?;
        Ok(response.json().await?)
    }
    pub async fn tickle(&self) -> Result<Value, reqwest::Error> {
        let response = self.client.post(self.get_url("/tickle")).header(reqwest::header::CONTENT_LENGTH, reqwest::header::HeaderValue::from_static("0")).body("").send().await?;
        Ok(response.json().await?)
    }
    pub async fn get_positions(&self, page: i32) -> Result<Vec<Position>, reqwest::Error> {
        let path = format!("/portfolio/{}/positions/{}", self.account, page);
        let response = self.client.get(self.get_url(&path)).body("").send().await?;
        println!("{:#?}", response.status());
        Ok(process_response(response).await?)
    }
    pub async fn get_session_id(mut self) -> Result<Self, reqwest::Error> {
        let response = self.tickle().await?;
        let as_obj = response.as_object().unwrap();
        self.session_id = Some(as_obj["session"].as_str().unwrap().to_string());
        Ok(self)
    }
    pub async fn get_security_definition_by_contract_id(&self, contract_ids: Vec<i32>) -> Result<SecurityDefinitions, reqwest::Error> {
        let path = "/trsrv/secdef";
        let payload = json!({
            "conids" : contract_ids,
        });
        let response = self.client.post(self.get_url(&path)).json(&payload).send().await?;
        Ok(process_response(response).await?)
    }
    pub async fn get_futures_by_symbol(&self, symbols: Vec<&str>) -> Result<FuturesContracts, reqwest::Error> {
        let path = "/trsrv/futures";
        let request = self.client.get(self.get_url(&path)).query(&[("symbols", symbols.join(","))]);
        let response = request.send().await?;
        Ok(process_response(response).await?)
    }
    pub async fn get_stocks_by_symbol(&self, symbols: Vec<&str>) -> Result<StockContracts, reqwest::Error> {
        let path = "/trsrv/stocks";
        let request = self.client.get(self.get_url(&path)).query(&[("symbols", symbols.join(","))]);
        let response = request.send().await?;
        Ok(process_response(response).await?)
    }
}

