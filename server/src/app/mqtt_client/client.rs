use crate::common::config::MqttConfig;
use crate::system_control::control_volume::{AudioControl, VolumeControl};
use crate::system_control::operate::launch_app_with_to;
use anyhow::Result;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS, SubscribeFilter};
use std::time::Duration;
use tokio::sync::mpsc;

// 定义消息结构，用于在任务间传递控制指令
struct ControlMessage {
    topic: String,
    payload: String,
}
struct MqttPublishRequest {
    topic: String,
    payload: String,
}

pub async fn start_mqtt(mqtt_config: &MqttConfig) -> Result<()> {
    // 1. 配置 MQTT 连接选项
    let mut mqtt_options =
        MqttOptions::new(&mqtt_config.client_id, &mqtt_config.host, mqtt_config.port);
    mqtt_options.set_keep_alive(Duration::from_secs(20));
    if (mqtt_config.username.is_some() && mqtt_config.password.is_some()) {
        mqtt_options.set_credentials(
            mqtt_config.username.as_ref().unwrap(),
            mqtt_config.password.as_ref().unwrap(),
        );
    }
    mqtt_options.set_clean_session(true);

    // 2. 创建 AsyncClient 和事件接收器
    let (client, mut event_loop) = AsyncClient::new(mqtt_options, 10);
    let client_clone = client.clone();
    // 【新增】创建用于从控制线程向主任务发送“发布请求”的通道
    let (pub_tx, mut pub_rx) = mpsc::channel::<MqttPublishRequest>(100);
    let pub_tx_clone = pub_tx.clone(); // 克隆一份给控制线程使用
    // 3. 创建通道用于发送控制指令
    let (tx, mut rx) = mpsc::channel::<ControlMessage>(100);
    let tx_clone = tx.clone();
    // 4. 启动一个阻塞任务或在线程中处理音量控制（因为 VolumeControl 不是 Send 的）
    // 注意：COM 对象通常需要在创建它的线程上访问。
    let control_handle = std::thread::spawn(move || {
        // 在新线程中初始化 COM 库 (如果需要)
        match VolumeControl::new() {
            Ok(mut volume_control) => {
                while let Some(msg) = rx.blocking_recv() {
                    let payload = serde_json::from_str::<serde_json::Value>(&msg.payload)
                        .expect("无法解析 JSON");
                    println!("topic and payload:{}_{}", msg.topic, msg.payload);
                    if msg.topic == "tv-web/control/volume" {
                        if payload["mute"].is_string() && payload["mute"] == "switch" {
                            let current_mute = volume_control.get_mute().unwrap();
                            if let Err(e) = volume_control.set_mute(!current_mute) {
                                log::error!("设置静音失败: {:?}", e);
                            } else {
                                log::info!("处理静音控制: {}", msg.payload);
                                let new_volume_muted = volume_control.get_mute().unwrap();
                                let new_volume = volume_control.get_volume().unwrap();
                                let state = serde_json::json!({
                                    "volume": new_volume,
                                    "mute": new_volume_muted
                                });
                                if pub_tx_clone
                                    .blocking_send(MqttPublishRequest {
                                        topic: "tv-web/volume/state/send".to_string(),
                                        payload: state.to_string(),
                                    })
                                    .is_err()
                                {
                                    log::error!("无法将状态推送到 MQTT");
                                };
                            }
                        } else if payload["volume"].is_number() {
                            let volume = payload["volume"].as_u64().unwrap() as u8;
                            if let Err(e) = volume_control.set_volume(volume) {
                                log::error!("设置音量失败: {:?}", e);
                            } else {
                                log::info!("处理音量控制: {}", msg.payload);
                                let new_volume_muted = volume_control.get_mute().unwrap();
                                let new_volume = volume_control.get_volume().unwrap();
                                let state = serde_json::json!({
                                    "volume": new_volume,
                                    "mute": new_volume_muted
                                });
                                if pub_tx_clone
                                    .blocking_send(MqttPublishRequest {
                                        topic: "tv-web/volume/state/send".to_string(),
                                        payload: state.to_string(),
                                    })
                                    .is_err()
                                {
                                    log::error!("无法将状态推送到 MQTT");
                                };
                            }
                        }
                    } else if msg.topic == "tv-web/volume/state/receive" {
                        let current_volume = volume_control.get_volume().unwrap();
                        let current_mute = volume_control.get_mute().unwrap();
                        let state =
                            serde_json::json!({ "volume": current_volume, "mute": current_mute });
                        // push to mqtt
                        if pub_tx_clone
                            .blocking_send(MqttPublishRequest {
                                topic: "tv-web/volume/state/send".to_string(),
                                payload: state.to_string(),
                            })
                            .is_err()
                        {
                            log::error!("无法将状态推送到 MQTT");
                        };
                    } else if msg.topic == "tv-web/control/launch_app" {
                        if payload["app_name"].is_string() {
                            let app_name = payload["app_name"].as_str().unwrap();
                            if let Err(e) = launch_app_with_to(app_name) {
                                log::error!("启动应用程序失败: {:?}", e);
                            } else {
                                log::info!("asdasd")
                            }
                        }
                    } else {
                        log::warn!("未处理的主题: {}", msg.topic);
                    }
                }
            }
            Err(e) => {
                log::error!("初始化音量控制失败: {:?}", e);
            }
        }
    });

    // 5. 启动 MQTT 事件循环任务 (只负责接收和转发，不涉及非 Send 对象)
    tokio::spawn(async move {
        loop {
            tokio::select! {
                // 1. 监听 MQTT 网络事件 (接收消息)
                notification = event_loop.poll() => {
                    match notification {
                        Ok(Event::Incoming(Packet::Publish(p))) => {
                            let topic = p.topic.clone();
                            let payload = String::from_utf8_lossy(&p.payload).to_string();

                            if tx_clone.send(ControlMessage { topic, payload }).await.is_err() {
                                log::error!("控制通道已关闭");
                                break;
                            }
                        }
                        Ok(_) => {} // 忽略其他事件 (如 Connect, SubAck 等)
                        Err(e) => {
                            log::error!("MQTT 事件循环错误: {:?}", e);
                            break;
                        }
                    }
                },

                // 2. 监听来自控制线程的发送请求
                Some(req) = pub_rx.recv() => {
                    log::info!("正在发送 MQTT 消息到主题: {}", req.topic);
                    if let Err(e) = client_clone.publish(&req.topic, QoS::AtLeastOnce, false, req.payload).await {
                        log::error!("发布 MQTT 消息失败: {:?}", e);
                    } else {
                        log::info!("MQTT 消息发送成功");
                    }
                }

                // 如果所有通道都关闭了，退出循环
                else => break,
            }
        }
    });

    println!("订阅主题");

    // 6. 订阅主题
    let topics = vec![
        SubscribeFilter::new("tv-web/control/volume".to_string(), QoS::AtLeastOnce),
        SubscribeFilter::new("tv-web/volume/state/receive".to_string(), QoS::AtLeastOnce),
        // SubscribeFilter::new("tv-web/volume/state/send".to_string(), QoS::AtLeastOnce),
        SubscribeFilter::new("tv-web/control/launch_app".to_string(), QoS::AtMostOnce),
    ];

    client.subscribe_many(topics).await?;
    log::info!("已订阅主题");

    // 7. 等待退出信号
    tokio::signal::ctrl_c().await?;
    log::info!("接收到退出信号，断开连接");

    client.disconnect().await.ok();

    // 等待控制线程结束
    drop(tx); // 关闭通道，使接收线程退出
    control_handle.join().ok();

    Ok(())
}
