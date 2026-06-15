use crate::common::config::MqttConfig;
use crate::system::control_volume::{AudioControl, VolumeControl};
use crate::system::operate::launch_app;
use anyhow::Result;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS, SubscribeFilter};
use std::time::Duration;
use tokio::sync::mpsc;

// 定义消息结构，用于在任务间传递控制指令
struct ControlMessage {
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

                    if msg.topic == "tv-web/control/volume" {
                        if payload["mute"].is_string() && payload["mute"] == "switch" {
                            let current_mute = volume_control.get_mute().unwrap();
                            if let Err(e) = volume_control.set_mute(!current_mute) {
                                log::error!("设置静音失败: {:?}", e);
                            } else {
                                log::info!("处理静音控制: {}", msg.payload);
                            }
                        } else if payload["volume"].is_number() {
                            let volume = payload["volume"].as_u64().unwrap() as u8;
                            if let Err(e) = volume_control.set_volume(volume) {
                                log::error!("设置音量失败: {:?}", e);
                            } else {
                                log::info!("处理音量控制: {}", msg.payload);
                            }
                        }
                    } else if msg.topic == "tv-web/control/launch_app" {
                        if payload["app_name"].is_string() {
                            let app_name = payload["app_name"].as_str().unwrap();
                            if let Err(e) = launch_app(app_name) {
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
            match event_loop.poll().await {
                Ok(notification) => {
                    if let Event::Incoming(Packet::Publish(p)) = notification {
                        let topic = p.topic.clone();
                        let payload = String::from_utf8_lossy(&p.payload).to_string();

                        // 发送控制消息到处理线程
                        if tx_clone
                            .send(ControlMessage { topic, payload })
                            .await
                            .is_err()
                        {
                            log::error!("控制通道已关闭");
                            break;
                        }
                    }
                }
                Err(e) => {
                    log::error!("MQTT 事件循环错误: {:?}", e);
                    break;
                }
            }
        }
    });

    println!("订阅主题");

    // 6. 订阅主题
    let topics = vec![
        SubscribeFilter::new("tv-web/control/volume".to_string(), QoS::AtLeastOnce),
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
