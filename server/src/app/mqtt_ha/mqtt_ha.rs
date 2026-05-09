use crate::app::mqtt_ha::control_mqtt::handle_control_online_status;
use crate::app::mqtt_ha::discover_mqtt::{
    generate_common_device_entity, generate_computer_cpu_info, generate_computer_memory_info,
    generate_computer_online_status, generate_computer_os_info,
    generate_control_online_status_payload, generate_launch_app_payload, merge_ha_configs,
};
use crate::common::config::MqttConfig;
use crate::system::operate::{get_system_info_json, launch_app};
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS, SubscribeFilter};
use std::time::Duration;

pub async fn start_mqtt(mqtt_config: &MqttConfig) {
    let mut mqtt_options = MqttOptions::new(
        mqtt_config.client_id.clone(), // 客户端ID（随便写，唯一即可）
        mqtt_config.host.clone(),      // 本地 MQTT 服务器地址
        mqtt_config.port,              // 默认端口
    );
    if let (Some(username), Some(password)) = (&mqtt_config.username, &mqtt_config.password) {
        if !username.is_empty() {
            println!("正在配置 MQTT 认证信息...");
            mqtt_options.set_credentials(username, password);
        }
    }
    mqtt_options.set_keep_alive(Duration::from_secs(mqtt_config.interval));
    let will_topic = format!(
        "ha_state/{}/common/availability_status_online",
        mqtt_config.client_id
    );
    // ====================遗嘱消息===================
    // 2. 定义遗嘱内容 (HA 通常识别 "offline" 或 JSON {"state": "offline"})
    let will_payload = serde_json::json!({
        "available": "OFF"
    })
    .to_string();

    // 3. 配置遗嘱选项
    // 参数: 主题, payload, QoS, retain (是否保留消息)
    // retain 设为 true 很重要，这样 HA 重启后也能立刻知道设备是离线的
    mqtt_options.set_last_will(rumqttc::LastWill::new(
        will_topic,
        will_payload,
        QoS::AtLeastOnce,
        true,
    ));
    // ====================== 2. 创建客户端 ======================
    let (client, mut event_loop) = AsyncClient::new(mqtt_options, 10);
    // 等待连接建立
    let connect_timeout = Duration::from_secs(mqtt_config.expire_time); // 设置一个合理的超时时间
    let start_time = std::time::Instant::now();
    let mut connected = false;
    // tokio::time::sleep(Duration::from_millis(mqtt_config.expire_time)).await;

    while start_time.elapsed() < connect_timeout {
        match event_loop.poll().await {
            Ok(notification) => {
                if let Event::Incoming(Packet::ConnAck(_)) = notification {
                    connected = true;
                    println!("✅ MQTT 连接成功");
                    break;
                }
            }
            Err(e) => {
                eprintln!("❌ MQTT 连接过程中出错: {:?}", e);
                // 可以选择 return 退出函数，或者继续重试
                return;
            }
        }
    }

    if !connected {
        eprintln!("❌ MQTT 连接超时");
        return;
    }
    // 异步线程处理 MQTT 事件（必须运行）
    let control_online_topic = format!(
        "ha_control/{}/control/set_status_online",
        mqtt_config.client_id
    );
    let launch_app_topic = format!("ha_control/{}/control/launch_app", mqtt_config.client_id);
    tokio::spawn(async move {
        loop {
            match event_loop.poll().await {
                Ok(notification) => {
                    if let Event::Incoming(Packet::Publish(p)) = notification {
                        let payload_str = String::from_utf8_lossy(&p.payload);
                        // 1. 判断是否是控制主题
                        if p.topic == control_online_topic {
                            handle_control_online_status(payload_str.to_string().as_str())
                        } else if p.topic == launch_app_topic {
                            println!("launch app:{}", payload_str);
                            match launch_app(&payload_str) {
                                Ok(_) => println!("应用启动命令已发送"),
                                Err(e) => eprintln!("启动应用失败: {:?}", e),
                            }
                        } else {
                            // 其他主题的消息日志
                            println!("📩 收到消息 [{}]: {}", p.topic, payload_str);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("MQTT 事件循环错误: {:?}", e);
                    break; // 出错时退出循环，避免无限打印错误
                }
            }
        }
    });

    // ====================== 3. 订阅主题 ======================
    // NOTE:控制电脑的消息在这里订阅
    let launch_apps_topics = Vec::from([format!(
        "ha_control/{}/control/launch_app",
        mqtt_config.client_id
    )]);
    let control_computer_topics = Vec::from([format!(
        "ha_control/{}/control/set_status_online",
        mqtt_config.client_id
    )]);

    let all_subscribe_topic: Vec<String> = launch_apps_topics
        .iter()
        .chain(control_computer_topics.iter())
        .cloned() // 因为 iter() 产生的是 &String，需要 cloned() 变成 String
        .collect();
    let subscribe_filters: Vec<SubscribeFilter> = all_subscribe_topic
        .into_iter()
        .map(|topic| SubscribeFilter::new(topic, QoS::AtLeastOnce))
        .collect();
    match client.subscribe_many(subscribe_filters).await {
        Ok(_) => println!("成功订阅所有控制主题"),
        Err(e) => eprintln!("订阅失败: {:?}", e),
    }
    // ====================== 2. 发送设备信息 ======================

    let common_device_entity = generate_common_device_entity(mqtt_config);
    let mut all_ha_configs = serde_json::Map::new();
    merge_ha_configs(
        &mut all_ha_configs,
        generate_computer_online_status(mqtt_config, &common_device_entity),
    );
    merge_ha_configs(
        &mut all_ha_configs,
        generate_computer_os_info(mqtt_config, &common_device_entity),
    );
    merge_ha_configs(
        &mut all_ha_configs,
        generate_computer_cpu_info(mqtt_config, &common_device_entity),
    );
    merge_ha_configs(
        &mut all_ha_configs,
        generate_computer_memory_info(mqtt_config, &common_device_entity),
    );
    merge_ha_configs(
        &mut all_ha_configs,
        generate_control_online_status_payload(mqtt_config, &common_device_entity),
    );
    merge_ha_configs(
        &mut all_ha_configs,
        generate_launch_app_payload(mqtt_config, &common_device_entity),
    );

    for (topic, payload) in &all_ha_configs {
        client
            .publish(topic, QoS::AtLeastOnce, false, payload.to_string())
            .await
            .unwrap();
    }
    // ====================== 4. 遗嘱消息 ======================

    // ====================== 5. 发布消息 ======================
    let mut count = 0;
    loop {
        count += 1;
        let computer_info = get_system_info_json();
        match computer_info {
            Some(value) => {
                let os_info = value.os;
                let cpu_info = value.cpu;
                let memory_info = value.memory;
                let online_status = serde_json::json!({
                    "state": "ON",
                });
                let control_online_status = serde_json::json!({
                    "state":"ON",
                    "available":"ON",
                });

                // let availability_status_online = "online".to_string();
                let all_info = serde_json::json!({
                    format!("ha_state/{}/info/os", mqtt_config.client_id): os_info,
                    format!("ha_state/{}/info/cpu", mqtt_config.client_id): cpu_info,
                    format!("ha_state/{}/info/memory", mqtt_config.client_id): memory_info,
                    format!("ha_state/{}/info/status_online", mqtt_config.client_id): online_status,
                    format!("ha_state/{}/control/status_online", mqtt_config.client_id): control_online_status,
                    format!("ha_state/{}/common/availability_status_online", mqtt_config.client_id): control_online_status,
                });
                // 遍历发送
                for (topic, payload) in all_info.as_object().unwrap() {
                    client
                        .publish(topic, QoS::AtLeastOnce, false, payload.to_string())
                        .await
                        .unwrap();
                }
            }
            None => println!("None"),
        }
        println!("已发送：第 {} 条消息", count);

        tokio::time::sleep(Duration::from_secs(mqtt_config.interval)).await;
    }
}
