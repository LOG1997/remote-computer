use axum::{
    extract::{
        ConnectInfo, Query,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::Response,
};
use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use http::HeaderMap;

use crate::common::models::QueryAuth;

pub async fn browser_service_handler(
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    Query(token): Query<QueryAuth>,
) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();
    tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    println!("text is {}", text);
                    sender.send(Message::Text(text)).await.ok();
                }
                Message::Binary(data) => {
                    println!("binary is {:?}", data);
                    sender.send(Message::Binary(data)).await.ok();
                }
                _ => {
                    println!("unknown message type");
                }
            }
        }
    })
    .await
    .ok();
}
