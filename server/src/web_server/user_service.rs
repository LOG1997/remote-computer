use anyhow::Result;
use axum::{
    Json,
    extract::{
        ConnectInfo, Query,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::Response,
};
use chrono::Utc;
use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use http::HeaderMap;
use rustls::crypto::cipher::NONCE_LEN;
use serde::de::value;
use serde_json::{Map, Value, json};

use crate::{
    common::models::{MsgReqModel, MsgRspModel, MsgType, ParamValue, QueryAuth},
    system_control::{
        info::get_system_info_json,
        operate::{execute_reboot, execute_shutdown},
    },
};

pub async fn user_service_handler(
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    Query(token): Query<QueryAuth>,
) -> Response {
    println!("开始websocket连接");
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();
    tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    let json_msg = handle_msg(text.as_str()).await;
                    let message_text = serde_json::to_string(&json_msg).unwrap_or_default();
                    sender.send(Message::Text(message_text.into())).await.ok();
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

async fn handle_msg(text: &str) -> MsgRspModel<Value> {
    let req = match parse_message(text) {
        Ok(value) => value,
        Err(e) => {
            return MsgRspModel::error(MsgType::Error, Some(e.to_string()));
        }
    };
    let topic = req.topic;
    let command = req.command;
    match topic {
        MsgType::SystemControl => {
            println!("is systemcontrol");
            match command {
                Some(value) => {
                    let command_type = value.command_type;
                    let command_param = value.param;
                    match command_type.as_str() {
                        "shutdown" | "reboot" => {
                            let immediate: bool = match command_param {
                                None => false,
                                Some(ParamValue::Bool(b)) => b,
                                Some(_) => {
                                    return MsgRspModel::error(
                                        MsgType::Error,
                                        Some("param请输入bool值".to_string()),
                                    );
                                }
                            };
                            if command_type == "shutdown" {
                                execute_shutdown(immediate);
                            }
                            if command_type == "reboot" {
                                execute_reboot(immediate);
                            }
                            println!("关机和重启命令：{immediate:?}");
                            todo!("关机和重启命令")
                        }
                        _ => {
                            println!("nnn");
                        }
                    };
                }
                None => {}
            }
        }
        MsgType::GetSystemInfo => match command {
            Some(value) => {
                let command_type = value.command_type;
                let command_param = value.param;
                match command_type.as_str() {
                    "get_system_info" => {
                        println!("获取系统信息");
                        let system_info = get_system_info_json().unwrap();
                        return MsgRspModel::success(topic, json!(system_info), None);
                    }
                    _ => {
                        println!("nnn");
                    }
                };
            }
            None => {}
        },
        MsgType::GetVolume => {}
        MsgType::LaunchApp => match command {
            Some(value) => {
                let command_type = value.command_type;
                let command_param = value.param;
                match command_type.as_str() {
                    "launch" => {
                        let app_name: String = match command_param {
                            None => {
                                return MsgRspModel::error(
                                    MsgType::Error,
                                    Some("请输入需要启动的app名称".to_string()),
                                );
                            }
                            Some(ParamValue::String(s)) => s,
                            Some(_) => {
                                return MsgRspModel::error(
                                    MsgType::Error,
                                    Some("您输入的应用名称格式不对，请输入字符串".to_string()),
                                );
                            }
                        };
                        println!("启动应用：{app_name:?}");
                        todo!("启动应用{app_name}")
                    }
                    "exit" => {
                        let app_name: String = match command_param {
                            None => {
                                return MsgRspModel::error(
                                    MsgType::Error,
                                    Some("请输入需要退出的app名称".to_string()),
                                );
                            }
                            Some(ParamValue::String(s)) => s,
                            Some(_) => {
                                return MsgRspModel::error(
                                    MsgType::Error,
                                    Some("您输入的应用名称格式不对，请输入字符串".to_string()),
                                );
                            }
                        };
                        println!("退出应用：{app_name:?}");
                        todo!("退出应用{app_name}")
                    }
                    _ => {
                        println!("nnnnn")
                    }
                }
            }
            None => {}
        },
        MsgType::BrowserControl => match command {
            Some(value) => {
                let command_type = value.command_type;
                let command_param = value.param;
                match command_type.as_str() {
                    "bilibili" => {
                        println!("向bilibili发送消息");
                        todo!("向bilibili发送消息")
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
