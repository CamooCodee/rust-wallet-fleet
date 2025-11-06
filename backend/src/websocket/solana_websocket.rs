use std::{collections::HashMap, sync::Arc};

use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;
use tokio::{
    net::TcpStream,
    sync::{Mutex, Notify},
};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};
use tungstenite::{Message, client::IntoClientRequest};

type WsWrite = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type WsRead = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

#[derive(Deserialize)]
struct RpcSubscriptionResponse {
    result: u64,
    id: String,
}

#[derive(Deserialize)]
struct RpcResponse {
    params: RpcResponseParams,
}

#[derive(Deserialize)]
struct RpcResponseParams {
    subscription: u64,
}

pub struct SolanaWebsocket {
    address: String,
    ws_write: Arc<Mutex<Option<WsWrite>>>,
    subs_to_id: Arc<Mutex<HashMap<u64, String>>>,
    subscriptions_results: Arc<Mutex<HashMap<String, String>>>,
    subscriptions: Arc<Mutex<HashMap<String, Arc<Notify>>>>,
}

impl SolanaWebsocket {
    pub async fn new(address: &str) -> Self {
        let mut ws = SolanaWebsocket {
            address: address.to_owned(),
            ws_write: Arc::new(Mutex::new(None)),
            subs_to_id: Arc::new(Mutex::new(HashMap::new())),
            subscriptions_results: Arc::new(Mutex::new(HashMap::new())),
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
        };

        let writer = ws.connect().await;
        let writer_arc = Arc::clone(&ws.ws_write);
        let writer_ptr = &mut *writer_arc.lock().await;
        *writer_ptr = Some(writer);

        ws
    }

    async fn connect(&mut self) -> WsWrite {
        let request = self.address.clone().into_client_request().unwrap();
        let conn_result = connect_async(request).await;
        let (stream, res) = match conn_result {
            Err(err) => {
                panic!("Error connecting to websocket {}", err);
            }
            Ok(v) => v,
        };

        if res.status() == StatusCode::SWITCHING_PROTOCOLS {
            println!("Established websocket connection successfully.");
        }

        let (write, read) = stream.split();

        let subs_to_id_arc = self.subs_to_id.clone();
        subs_to_id_arc.lock().await.clear();
        let subscriptions_results_arc = self.subscriptions_results.clone();
        subscriptions_results_arc.lock().await.clear();
        let subscriptions_arc = self.subscriptions.clone();
        subscriptions_arc.lock().await.clear();

        self.spawn_reader(read);

        write
    }

    fn spawn_reader(&self, ws_read: WsRead) {
        let subs_to_id = Arc::clone(&self.subs_to_id);
        let subscriptions = Arc::clone(&self.subscriptions);
        let ws_write = Arc::clone(&self.ws_write);
        tokio::spawn(async move {
            ws_read
                .for_each(|received| async {
                    let message = match received {
                        Err(err) => {
                            if matches!(err, tungstenite::Error::ConnectionClosed) {
                                eprintln!("Connection closed")
                            }
                            eprintln!("Error receiving websocket data: {err}");
                            return;
                        }
                        Ok(msg) => msg,
                    };

                    let message_res = message.into_text().unwrap();
                    let response = serde_json::from_str::<RpcSubscriptionResponse>(&message_res);
                    if let Ok(subscription) = response {
                        let mut subs_to_id = subs_to_id.lock().await;
                        subs_to_id.insert(subscription.result, subscription.id);
                        return;
                    }

                    let response = serde_json::from_str::<RpcResponse>(&message_res);
                    if let Ok(data) = response {
                        let subs_to_id = subs_to_id.lock().await;
                        let subscriptions = subscriptions.lock().await;
                        let id = subs_to_id
                            .get(&data.params.subscription)
                            .expect("got unexpected message from websocket");
                        let notify = &subscriptions[id];
                        notify.notify_waiters();
                    }
                })
                .await;

            let mut ws_write_locked = ws_write.lock().await;
            *ws_write_locked = None;
        });
    }

    pub async fn confirm_transaction(&mut self, signature: &str) {
        let id = &signature[..12];
        let message_data = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": "signatureSubscribe",
            "params": [
                signature,
                {
                    "commitment": "confirmed"
                }
            ]
        });
        let data_str = message_data.to_string();
        let message = Message::text(data_str);
        {
            let writer_arc = Arc::clone(&self.ws_write);
            let writer = &mut *writer_arc.lock().await;
            if matches!(writer, None) {
                *writer = Some(self.connect().await);
                if matches!(writer, None) {
                    eprintln!(
                        "Failed to confirm transaction, no websocket writer after reconnecting."
                    );
                    return;
                }
            }
            let writer = writer.as_mut().unwrap();
            let send_result = writer.send(message).await;

            if let Err(err) = send_result {
                eprintln!("Error sending: {err}");
            }
        }

        let notify = Arc::new(Notify::new());
        {
            let mut subcriptions = self.subscriptions.lock().await;
            subcriptions.insert(id.to_owned(), notify.clone());
        }

        println!("Confirming transaction {}", &signature[..6]);

        notify.notified().await;
        let mut subscriptions = self.subscriptions.lock().await;
        subscriptions.remove(id);
    }
}
