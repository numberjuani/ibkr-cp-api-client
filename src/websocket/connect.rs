use crate::client::IBClientPortal;
use futures_util::{
    stream::{SplitSink, SplitStream},
    StreamExt, SinkExt,
};

use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    tungstenite::{Error, Message},
    MaybeTlsStream, WebSocketStream,
};
pub type WriteWs = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
pub type ReadWs = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
use crate::websocket::connect::Message::Text;

use super::requests::Subscription;
//https://interactivebrokers.github.io/cpwebapi/websockets

pub async fn listen(reader: &mut ReadWs, on_message: fn(String) -> ()) -> Result<(), Error> {
    while let Some(msg) = reader.next().await {
        on_message(msg?.into_text()?);
    }
    Ok(())
}

impl IBClientPortal {
    fn get_ws_url(&self) -> String {
        let base = if self.listen_ssl { "wss" } else { "ws" };
        format!("{base}://localhost:{}/v1/api/ws", self.port)
    }
    fn ws_auth_msg(&self) -> String {
        let session = self.session_id.clone().unwrap();
        json!({"session":session}).to_string()
    }
    pub async fn connect_to_websocket(&self, subscriptions:Vec<Subscription>,on_message: fn(String) -> ()) -> Result<(), Error> {
        let url = self.get_ws_url();
        let (ws_stream, _) = tokio_tungstenite::connect_async(url).await?;
        let (mut ws_out, mut ws_in) = ws_stream.split();
        let auth = self.ws_auth_msg();
        ws_out.send(Text(auth.to_owned())).await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        for sub in subscriptions {
            ws_out.send(Text(sub.build())).await?;
        }
        tokio::try_join!(listen(&mut ws_in, on_message),)?;
        Ok(())
    }
}
