use anyhow::Result;
use axum::{
    Json,
    extract::{
        ConnectInfo, Query, State,
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
use serde_json::{Map, Value, json};
use tokio::sync::{mpsc::UnboundedSender, oneshot};
use tracing::{error, info, instrument, trace, warn};

use crate::{
    common::models::{
        AppState, AudioCommand, MsgReqModel, MsgRspModel, MsgType, ParamValue, QueryAuth,
        SecurityConfig,
    },
    system_control::{
        info::{self, get_system_info_json},
        operate::{execute_reboot, execute_shutdown, launch_app_with_to},
    },
};

pub async fn user_service_handler(
    State(app_state): State<AppState>,
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    Query(token): Query<QueryAuth>,
) -> Response {
    println!("开始websocket连接");

    ws.on_upgrade(|socket| handle_socket(socket, app_state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let audio_tx = state.audio_tx;
    let config = state.config;
    let security_config = config.security;
    let (mut sender, mut receiver) = socket.split();
    tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    let json_msg =
                        handle_msg(text.as_str(), security_config.clone(), audio_tx.clone()).await;
                    let message_text = serde_json::to_string(&json_msg).unwrap_or_default();
                    sender.send(Message::Text(message_text.into())).await.ok();
                }
                Message::Binary(data) => {
                    println!("binary is {:?}", data);
                    sender.send(Message::Binary(data)).await.ok();
                }
                _ => {
                    println!("unknown message type：{msg:?}");
                }
            }
        }
    })
    .await
    .ok();
}

#[instrument]
async fn handle_msg(
    text: &str,
    security_config: SecurityConfig,
    audio_tx: UnboundedSender<AudioCommand>,
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
    info!("topic is {topic:?}");
    match topic {
        MsgType::SystemControl => match command {
            Some(value) => {
                let command_type = value.command_type;
                let command_param = value.param;
                match command_type.as_str() {
                    "shutdown" | "reboot" => {
                        let immediate = match command_param {
                            None => false,
                            Some(Value::Object(b)) => {
                                let psd = b.get("password");
                                let imt = b.get("immediate");
                                match psd {
                                    Some(psd_value) => {
                                        if security_config.shutdown_key != psd_value.to_owned() {
                                            warn!("password is wrong");
                                            return MsgRspModel::error(
                                                MsgType::Error,
                                                Some("密码错误".to_string()),
                                            );
                                        }
                                    }
                                    None => {
                                        warn!("user's password is empty");
                                        return MsgRspModel::error(
                                            MsgType::Error,
                                            Some("没输入密码".to_string()),
                                        );
                                    }
                                }
                                match imt {
                                    Some(imt_value) => {
                                        if let Some(imt_bool) = imt_value.as_bool() {
                                            imt_bool
                                        } else {
                                            false
                                        }
                                    }
                                    None => false,
                                }
                            }
                            Some(_) => {
                                warn!("user's input format is wrong");
                                return MsgRspModel::error(
                                    MsgType::Error,
                                    Some("param请输入bool值".to_string()),
                                );
                            }
                        };

                        if command_type == "shutdown" {
                            info!("execute shutdown");
                            execute_shutdown(immediate);
                        }
                        if command_type == "reboot" {
                            info!("execute reboot");
                            execute_reboot(immediate);
                        }
                    }
                    _ => {
                        warn!("command is not match:{command_type:?}");
                        println!("nnn");
                    }
                };
            }
            None => {
                warn!("command is empty");
            }
        },
        MsgType::GetSystemInfo => match command {
            Some(value) => {
                let command_type = value.command_type;
                match command_type.as_str() {
                    "get_system_info" => {
                        let system_info = get_system_info_json();
                        if let Some(info_data) = system_info {
                            info!("get systeminfo success");
                            return MsgRspModel::success(topic, json!(info_data), None);
                        } else {
                            warn!("get systeminfo fail or data is empty");
                            return MsgRspModel::error(
                                MsgType::Error,
                                Some("未获取到系统信息".to_string()),
                            );
                        }
                    }
                    _ => {
                        warn!("command is not match:{command_type:?}");
                    }
                };
            }
            None => {
                warn!("command is empty");
            }
        },
        MsgType::GetVolume => {
            let (reply_tx, reply_rx) = oneshot::channel::<u8>();
            let volume_cmd = AudioCommand::GetVolume { reply: reply_tx };
            if let Err(e) = audio_tx.send(volume_cmd) {
                error!("volume is error:{e:?}");
                return MsgRspModel::error(MsgType::Error, Some(e.to_string()));
            }
            let current_volume: i8 =
                match tokio::time::timeout(std::time::Duration::from_secs(1), reply_rx).await {
                    Ok(Ok(vol)) => vol as i8,
                    Ok(Err(_)) => -1,
                    Err(_) => -1,
                };
            if current_volume < 0 {
                warn!("not get current volume");
                return MsgRspModel::error(MsgType::Error, Some("没有获取到系统音量".to_string()));
            }
            let volume_json = json!({"volume":current_volume});
            info!("get current volume success:{volume_json:?}");
            return MsgRspModel::success(topic, volume_json, None);
        }
        MsgType::SetVolume => match command {
            Some(value) => {
                let command_param = value.param;

                match command_param {
                    Some(new_volume_value) => {
                        let new_volume_i8 = new_volume_value.as_i64().unwrap_or(-1) as i8;
                        if new_volume_i8 < 0 {
                            warn!("user's input volume format is wrong");
                            return MsgRspModel::error(
                                MsgType::Error,
                                Some("数据格式化出错，请检查你的传入数据".to_string()),
                            );
                        }
                        let (reply_tx, reply_rx) = oneshot::channel::<u8>();
                        let volume_cmd = AudioCommand::SetVolume {
                            volume: new_volume_i8 as u8,
                            reply: reply_tx,
                        };
                        if let Err(e) = audio_tx.send(volume_cmd) {
                            return MsgRspModel::error(MsgType::Error, Some(e.to_string()));
                        }
                        let current_volume: i8 =
                            match tokio::time::timeout(std::time::Duration::from_secs(1), reply_rx)
                                .await
                            {
                                Ok(Ok(vol)) => vol as i8,
                                Ok(Err(_)) => -1,
                                Err(_) => -1,
                            };
                        if current_volume < 0 {
                            warn!("not get volume after set");
                            return MsgRspModel::error(
                                MsgType::Error,
                                Some("没有获取到系统音量".to_string()),
                            );
                        }
                        let volume_json = json!({"volume":current_volume});
                        info!("set volume success and get success: {volume_json:?}");
                        return MsgRspModel::success(topic, volume_json, None);
                    }
                    None => {
                        warn!("sorry ,but user is not input the volume that needed");
                        return MsgRspModel::error(
                            MsgType::Error,
                            Some("未传入需要设置的音量值".to_string()),
                        );
                    }
                }
            }
            None => {
                warn!("sorry ,but user is not input the volume that needed");
                return MsgRspModel::error(
                    MsgType::Error,
                    Some("未传入需要设置的音量".to_string()),
                );
            }
        },
        MsgType::LaunchApp => match command {
            Some(value) => {
                let command_type = value.command_type;
                let command_param = value.param;
                match command_type.as_str() {
                    "launch" => {
                        let app_name: String = match command_param {
                            None => {
                                warn!("user is not input app_name");
                                return MsgRspModel::error(
                                    MsgType::Error,
                                    Some("请输入需要启动的app名称".to_string()),
                                );
                            }
                            Some(Value::String(s)) => s,
                            Some(_) => {
                                warn!("user is not input app_name with correct format");
                                return MsgRspModel::error(
                                    MsgType::Error,
                                    Some("您输入的应用名称格式不对，请输入字符串".to_string()),
                                );
                            }
                        };
                        match launch_app_with_to(&app_name) {
                            Ok(()) => {
                                info!("launch app success:{app_name:?}");
                                return MsgRspModel::success(
                                    topic,
                                    json!(""),
                                    Some("启动成功".to_string()),
                                );
                            }
                            Err(e) => {
                                warn!("launch app fail:{app_name:?},this error is :{e:?}");
                                return MsgRspModel::error(
                                    MsgType::Error,
                                    Some("启动失败".to_string() + e.to_string().as_str()),
                                );
                            }
                        }
                    }
                    "exit" => {
                        let app_name: String = match command_param {
                            None => {
                                warn!("user is not input app_name");
                                return MsgRspModel::error(
                                    MsgType::Error,
                                    Some("请输入需要退出的app名称".to_string()),
                                );
                            }
                            Some(Value::String(s)) => s,
                            Some(_) => {
                                warn!("user is not input app_name with correct format");
                                return MsgRspModel::error(
                                    MsgType::Error,
                                    Some("您输入的应用名称格式不对，请输入字符串".to_string()),
                                );
                            }
                        };
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
