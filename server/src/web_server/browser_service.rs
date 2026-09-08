use std::sync::Arc;

use anyhow::Result;
use axum::{
    extract::{
        Query, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::Response,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use http::HeaderMap;
use serde_json::{Value, json};
use tokio::sync::Mutex;
use tracing::{error, info, instrument, trace, warn};

use crate::common::models::{
    AppState, MsgReqModel, MsgRspModel, MsgType, QueryAuth, SecurityConfig,
};

pub async fn browser_service_handler(
    State(app_state): State<AppState>,
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    Query(token): Query<QueryAuth>,
) -> Response {
    println!("开始浏览器websocket连接");

    ws.on_upgrade(|socket| handle_socket(socket, app_state, "/browser".to_string()))
}
async fn handle_socket(socket: WebSocket, state: AppState, path: String) {
    let config = state.config;
    let security_config = Arc::new(config.security);
    let launch_apps = Arc::new(config.launch_apps);
    let (sender, mut receiver) = socket.split();
    let sender_arc = Arc::new(Mutex::new(sender));
    let (_, mut user_rx) = (state.user_tx.clone(), state.user_tx.subscribe());

    let inner_msg_task = {
        let sender_inner = sender_arc.clone();
        tokio::spawn(async move {
            while let Ok(msg) = user_rx.recv().await {
                println!(
                    "[{}] Received from broadcast this is broswer, sending to client: {}",
                    path, msg
                );
                sender_inner
                    .lock()
                    .await
                    .send(Message::Text(msg.into()))
                    .await
                    .ok();
            }
        })
    };
    let out_msg_task = {
        let security = security_config.clone();
        let launch = launch_apps.clone();
        let sender_out = sender_arc.clone();
        tokio::spawn(async move {
            while let Some(Ok(msg)) = receiver.next().await {
                println!("askdlasdlkalskdklasdkl");

                match msg {
                    Message::Text(text) => {
                        let json_msg = handle_msg(text.as_str(), &security, &launch).await;
                        let message_text = serde_json::to_string(&json_msg).unwrap_or_default();
                        sender_out
                            .lock()
                            .await
                            .send(Message::Text(message_text.into()))
                            .await
                            .ok();
                        // sender.send(Message::Text(message_text.into())).await.ok();
                    }
                    Message::Binary(data) => {
                        println!("binary is {:?}", data);
                        sender_out
                            .lock()
                            .await
                            .send(Message::Binary(data))
                            .await
                            .ok();
                    }
                    _ => {
                        println!("unknown message type：{msg:?}");
                    }
                }
            }
        })
    };
    tokio::select! {
        _ = inner_msg_task => {},
        _ = out_msg_task => {},
    }
}

#[instrument]
async fn handle_msg(
    text: &str,
    security_config: &Arc<SecurityConfig>,
    launch_apps: &Arc<Option<serde_json::Value>>,
) -> MsgRspModel<Value> {
    info!("get ws msg");
    let req = match parse_message(text) {
        Ok(value) => value,
        Err(e) => {
            return MsgRspModel::error(MsgType::Error, Some(e.to_string()));
        }
    };
    let topic = req.topic;
    let command = req.command;
    info!("browserrr topic is {topic:?}");
    match topic {
        MsgType::BrowserControl => match command {
            Some(value) => {
                let command_type = value.command_type;
                let command_param = value.param;
                match command_type.as_str() {
                    "bilibili" => {
                        println!("向bilibili发送消息1111");
                        // todo!("向bilibili发送消息")
                    }
                    "douyin" => {
                        println!("向抖音发送消息");
                        todo!("向抖音发送消息")
                    }
                    _ => {
                        println!("nnnnn")
                    }
                }
            }
            None => {}
        },
        MsgType::Ping => {
            trace!("ws ping");
            println!("this is ping");
        }
        _ => {
            println!("no type");
        }
    }
    let result = MsgRspModel::success(topic, json!("128128182"), Some("ansdas".to_string()));

    result
}

fn parse_message(raw: &str) -> Result<MsgReqModel, String> {
    // 尝试反序列化，serde 会自动检查：
    // 1. 是否为合法的 JSON 语法
    // 2. 所有字段类型是否匹配（例如 date_time 必须是字符串）
    // 3. msg_type 是否能从 JSON 中的值（字符串或数字）转为 MsgType 枚举
    // 4. token 字段必须存在且是字符串
    // 5. command 字段可以缺失（因为 Option），但如果存在，类型必须是有效的 JSON Value
    match serde_json::from_str::<MsgReqModel>(raw) {
        Ok(req) => {
            // 到这里，数据100%符合结构体定义
            // 还可以做额外的业务校验（例如 token 是否为空，日期格式是否合法）
            if req.token.is_empty() {
                return Err("token 不能为空".to_string());
            }
            // 校验日期格式（假设你想要 RFC3339）
            if chrono::DateTime::from_timestamp(req.date_time, 0).is_none() {
                return Err("date_time 格式无效".to_string());
            }
            Ok(req)
        }
        Err(e) => {
            // 反序列化失败，说明不符合结构体
            Err(format!("JSON 不符合 MsgReqModel 格式: {}", e))
        }
    }
}
