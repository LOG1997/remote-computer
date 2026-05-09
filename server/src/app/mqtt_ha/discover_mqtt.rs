use crate::common::config::MqttConfig;
use serde_json::json;

pub fn generate_common_device_entity(mqtt_config: &MqttConfig) -> serde_json::Value {
    json!({
      "name": mqtt_config.device_name,
      "identifiers": [mqtt_config.client_id],
      "manufacturer": mqtt_config.device_name,
      "model": mqtt_config.device_model,
    })
}

pub fn generate_computer_online_status(
    mqtt_config: &MqttConfig,
    common_device_entity: &serde_json::Value,
) -> serde_json::Value {
    let online_payload = json!({
        "name": "在线状态",
        "unique_id": format!("{}_status_online", mqtt_config.client_id),
        "state_topic": format!("ha_state/{}/info/status_online", mqtt_config.client_id),
        "value_template": "{{ value_json.state }}",
        "payload_on": "ON",
        "payload_off": "OFF",
        "device_class": "connectivity",
        "availability_topic": format!("ha_state/{}/common/availability_status_online", mqtt_config.client_id),
        // 在线/离线 状态值，设备发送给ha的消息
        "payload_available": "ON",
        "payload_not_available": "OFF",
        "availability_template": "{{ value_json.available }}",
        "expire_after": mqtt_config.expire_time,
        "device": common_device_entity,
    });

    // 2. 动态生成 Key
    let topic_key = format!(
        "homeassistant/binary_sensor/{}_info_status_online/config",
        mqtt_config.client_id
    );

    // 3. 构建最终的对象 Map
    let mut map = serde_json::Map::new();
    map.insert(topic_key, online_payload);

    // 4. 返回 Value::Object
    serde_json::Value::Object(map)
}

pub fn generate_computer_cpu_info(
    mqtt_config: &MqttConfig,
    common_device_entity: &serde_json::Value,
) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    // CPU型号
    let cpu_brand_payload = json!({
        "name": "CPU",
        "unique_id": format!("{}_cpu_brand", mqtt_config.client_id),
        "state_topic": format!("ha_state/{}/info/cpu", mqtt_config.client_id),
        "value_template": "{{ value_json.brand }}",
        "availability_topic": format!("ha_state/{}/common/availability_status_online", mqtt_config.client_id),
        // 在线/离线 状态值，设备发送给ha的消息
        "payload_available": "ON",
        "payload_not_available": "OFF",
        "availability_template": "{{ value_json.available }}",
        "expire_after": mqtt_config.expire_time,
        "device": common_device_entity,
    });

    let cpu_brand_key = format!(
        "homeassistant/sensor/{}_info_cpu_brand/config",
        mqtt_config.client_id
    );

    map.insert(cpu_brand_key, cpu_brand_payload);
    // CPU核心数
    let cpu_physical_core_count_payload = json!({
        "name": "核心数",
        "unique_id": format!("{}_cpu_physical_core_count", mqtt_config.client_id),
        "state_topic": format!("ha_state/{}/info/cpu", mqtt_config.client_id),
        "value_template": "{{ value_json.physical_core_count }}",
        "availability_topic": format!("ha_state/{}/common/availability_status_online", mqtt_config.client_id),
        // 在线/离线 状态值，设备发送给ha的消息
        "payload_available": "ON",
        "payload_not_available": "OFF",
        "availability_template": "{{ value_json.available }}",
        "expire_after": mqtt_config.expire_time,
        "device": common_device_entity,
    });

    let cpu_physical_core_count_key = format!(
        "homeassistant/sensor/{}_info_cpu_physical_core_count/config",
        mqtt_config.client_id
    );

    map.insert(cpu_physical_core_count_key, cpu_physical_core_count_payload);
    // CPU线程数
    let cpu_total_core_count_payload = json!({
        "name": "线程数",
        "unique_id": format!("{}_cpu_total_core_count", mqtt_config.client_id),
        "state_topic": format!("ha_state/{}/info/cpu", mqtt_config.client_id),
        "value_template": "{{ value_json.total_core_count }}",
        "availability_topic": format!("ha_state/{}/common/availability_status_online", mqtt_config.client_id),
        // 在线/离线 状态值，设备发送给ha的消息
        "payload_available": "ON",
        "payload_not_available": "OFF",
        "availability_template": "{{ value_json.available }}",
        "expire_after": mqtt_config.expire_time,
        "device": common_device_entity,
    });

    let cpu_total_core_count_key = format!(
        "homeassistant/sensor/{}_info_cpu_total_core_count/config",
        mqtt_config.client_id
    );

    map.insert(cpu_total_core_count_key, cpu_total_core_count_payload);
    // CPU频率
    let cpu_frequency_payload = json!({
        "name": "频率",
        "unique_id": format!("{}_cpu_frequency", mqtt_config.client_id),
        "state_topic": format!("ha_state/{}/info/cpu", mqtt_config.client_id),
        "value_template": "{{ (value_json.frequency/ 1000) | round(2) }}",
        "unit_of_measurement": "GHz",
        "availability_topic": format!("ha_state/{}/common/availability_status_online", mqtt_config.client_id),
        // 在线/离线 状态值，设备发送给ha的消息
        "payload_available": "ON",
        "payload_not_available": "OFF",
        "availability_template": "{{ value_json.available }}",
        "expire_after": mqtt_config.expire_time,
        "device": common_device_entity,
    });

    let cpu_frequency_key = format!(
        "homeassistant/sensor/{}_info_cpu_frequency/config",
        mqtt_config.client_id
    );

    map.insert(cpu_frequency_key, cpu_frequency_payload);
    // CPU使用率
    let cpu_usage_payload = json!({
        "name": "CPU使用率",
        "unique_id": format!("{}_cpu_usage", mqtt_config.client_id),
        "state_topic": format!("ha_state/{}/info/cpu", mqtt_config.client_id),
        "value_template": "{{ value_json.usage | round(1) }}",
        "unit_of_measurement": "%",
        "availability_topic": format!("ha_state/{}/common/availability_status_online", mqtt_config.client_id),
        // 在线/离线 状态值，设备发送给ha的消息
        "payload_available": "ON",
        "payload_not_available": "OFF",
        "availability_template": "{{ value_json.available }}",
        "expire_after": mqtt_config.expire_time,
        "device": common_device_entity,
    });

    let cpu_usage_key = format!(
        "homeassistant/sensor/{}_info_cpu_usage/config",
        mqtt_config.client_id
    );

    map.insert(cpu_usage_key, cpu_usage_payload);
    serde_json::Value::Object(map)
}

pub fn generate_computer_memory_info(
    mqtt_config: &MqttConfig,
    common_device_entity: &serde_json::Value,
) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    // CPU型号
    let memory_available_memory_payload = json!({
        "name": "未使用内存",
        "unique_id": format!("{}_memory_available_memory", mqtt_config.client_id),
        "state_topic": format!("ha_state/{}/info/memory", mqtt_config.client_id),
        "value_template": "{{ (value_json.available_memory/1073741824) | round(1) }}",
        "unit_of_measurement": "GB",
        "availability_topic": format!("ha_state/{}/common/availability_status_online", mqtt_config.client_id),
        // 在线/离线 状态值，设备发送给ha的消息
        "payload_available": "ON",
        "payload_not_available": "OFF",
        "availability_template": "{{ value_json.available }}",
        "expire_after": mqtt_config.expire_time,
        "device": common_device_entity,
    });

    let memory_available_memory_key = format!(
        "homeassistant/sensor/{}_info_memory_available_memory/config",
        mqtt_config.client_id
    );

    map.insert(memory_available_memory_key, memory_available_memory_payload);
    let memory_used_memory_payload = json!({
        "name": "已使用内存",
        "unique_id": format!("{}_memory_used_memory", mqtt_config.client_id),
        "state_topic": format!("ha_state/{}/info/memory", mqtt_config.client_id),
        "value_template": "{{ (value_json.used_memory/1073741824)|round(1) }}",
        "unit_of_measurement": "GB",
        "expire_after": mqtt_config.expire_time,
        "device": common_device_entity,
    });

    let memory_used_memory_key = format!(
        "homeassistant/sensor/{}_info_memory_used_memory/config",
        mqtt_config.client_id
    );

    map.insert(memory_used_memory_key, memory_used_memory_payload);
    // CPU线程数
    let memory_usage_payload = json!({
        "name": "内存使用率",
        "unique_id": format!("{}_memory_usage", mqtt_config.client_id),
        "state_topic": format!("ha_state/{}/info/memory", mqtt_config.client_id),
        "value_template": "{{ value_json.usage }}",
        "unit_of_measurement": "%",
        "availability_topic": format!("ha_state/{}/common/availability_status_online", mqtt_config.client_id),
        // 在线/离线 状态值，设备发送给ha的消息
        "payload_available": "ON",
        "payload_not_available": "OFF",
        "availability_template": "{{ value_json.available }}",
        "expire_after": mqtt_config.expire_time,
        "device": common_device_entity,
    });

    let memory_usage_key = format!(
        "homeassistant/sensor/{}_info_memory_usage/config",
        mqtt_config.client_id
    );

    map.insert(memory_usage_key, memory_usage_payload);
    serde_json::Value::Object(map)
}

pub fn generate_computer_os_info(
    mqtt_config: &MqttConfig,
    common_device_entity: &serde_json::Value,
) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    // CPU型号
    let os_host_name_payload = json!({
        "name": "电脑名称",
        "unique_id": format!("{}_os_host_name", mqtt_config.client_id),
        "state_topic": format!("ha_state/{}/info/os", mqtt_config.client_id),
        "value_template": "{{ value_json.host_name }}",
        "availability_topic": format!("ha_state/{}/common/availability_status_online", mqtt_config.client_id),
        // 在线/离线 状态值，设备发送给ha的消息
        "payload_available": "ON",
        "payload_not_available": "OFF",
        "availability_template": "{{ value_json.available }}",
        "expire_after": mqtt_config.expire_time,
        "device": common_device_entity,
    });

    let os_host_name_key = format!(
        "homeassistant/sensor/{}_info_os_host_name/config",
        mqtt_config.client_id
    );

    map.insert(os_host_name_key, os_host_name_payload);
    // CPU核心数
    let os_name_payload = json!({
        "name": "系统名称",
        "unique_id": format!("{}_os_name", mqtt_config.client_id),
        "state_topic": format!("ha_state/{}/info/os", mqtt_config.client_id),
        "value_template": "{{ value_json.name }}",
        "availability_topic": format!("ha_state/{}/common/availability_status_online", mqtt_config.client_id),
        // 在线/离线 状态值，设备发送给ha的消息
        "payload_available": "ON",
        "payload_not_available": "OFF",
        "availability_template": "{{ value_json.available }}",
        "expire_after": mqtt_config.expire_time,
        "device": common_device_entity,
    });

    let os_name_key = format!(
        "homeassistant/sensor/{}_info_os_name/config",
        mqtt_config.client_id
    );

    map.insert(os_name_key, os_name_payload);
    // CPU线程数
    let os_os_version_payload = json!({
        "name": "系统版本",
        "unique_id": format!("{}_os_os_version", mqtt_config.client_id),
        "state_topic": format!("ha_state/{}/info/os", mqtt_config.client_id),
        "value_template": "{{ value_json.os_version }}",
        "availability_topic": format!("ha_state/{}/common/availability_status_online", mqtt_config.client_id),
        // 在线/离线 状态值，设备发送给ha的消息
        "payload_available": "ON",
        "payload_not_available": "OFF",
        "availability_template": "{{ value_json.available }}",
        "expire_after": mqtt_config.expire_time,
        "device": common_device_entity,
    });

    let os_os_version_key = format!(
        "homeassistant/sensor/{}_info_os_os_version/config",
        mqtt_config.client_id
    );

    map.insert(os_os_version_key, os_os_version_payload);
    serde_json::Value::Object(map)
}

pub fn generate_control_online_status_payload(
    mqtt_config: &MqttConfig,
    common_device_entity: &serde_json::Value,
) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    let json_state_on = r#"{"state":"ON"}"#;
    let json_state_off = r#"{"state":"OFF"}"#;
    let control_online_status_payload = json!(
        {
            "name": "电源",
            "unique_id": format!("{}_online_switch", mqtt_config.client_id),
            "device_class": "switch",
            "state_topic": format!("ha_state/{}/control/status_online", mqtt_config.client_id),
            "command_topic":format!("ha_control{}/control/set_status_online", mqtt_config.client_id),

            "value_template": "{{ value_json.state }}",
            // ha发送的控制消息状态值
            "payload_on": json_state_on,
            "payload_off": json_state_off,
            // 设备发送给ha的消息状态值
            "state_on": "ON",
            "state_off": "OFF",


            "availability_topic": format!("ha_state/{}/common/availability_status_online", mqtt_config.client_id),
            // 在线/离线 状态值，设备发送给ha的消息
            "payload_available": "ON",
            "payload_not_available": "OFF",
            "availability_template": "{{ value_json.available }}",

            "device": common_device_entity,
        }
    );
    let control_online_status_key = format!(
        "homeassistant/switch/{}_control_status_online/config",
        mqtt_config.client_id
    );
    map.insert(control_online_status_key, control_online_status_payload);
    serde_json::Value::Object(map)
}

pub fn generate_launch_app_payload(
    mqtt_config: &MqttConfig,
    common_device_entity: &serde_json::Value,
) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    let apps = &mqtt_config.launch_app;
    if let Some(apps_map) = apps.as_object() {
        for (app_key, app_name) in apps_map.iter() {
            let payload = json!({
            "name": app_name,
            "unique_id": format!("{}_launch_app_{}", mqtt_config.client_id,app_key),
            "command_topic":format!("ha_control/{}/control/launch_app", mqtt_config.client_id),
            // ha发送的控制消息状态值
            "payload_press": app_key,

            "availability_topic": format!("ha_state/{}/common/availability_status_online", mqtt_config.client_id),
            // 在线/离线 状态值，设备发送给ha的消息
            "payload_available": "ON",
            "payload_not_available": "OFF",
            "availability_template": "{{ value_json.available }}",

            "device": common_device_entity,
            });
            let key = format!(
                "homeassistant/button/{}_launch_app_{}/config",
                mqtt_config.client_id, app_key
            );
            map.insert(key, payload);
        }
    }
    serde_json::Value::Object(map)
}

pub fn merge_ha_configs(
    target: &mut serde_json::Map<String, serde_json::Value>,
    source: serde_json::Value,
) {
    if let Some(obj) = source.as_object() {
        target.extend(obj.iter().map(|(k, v)| (k.clone(), v.clone())));
    }
}
