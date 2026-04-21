use crate::common::app::get_app_dir;
use crate::common::config::AppConfig;
use crate::system;
use axum::{extract::Json, http::StatusCode};
use serde::Deserialize;
use system::operate::{execute_reboot, execute_shutdown, get_system_info_json};

#[derive(Deserialize)]
pub struct ShutdownRequest {
    key: String,
    immediate: bool,
}

// 处理关机请求
pub async fn shutdown_handler(
    Json(body): Json<ShutdownRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let app_dir = get_app_dir();
    let app_config = AppConfig::from_file(AppConfig::default_path(&app_dir).to_str().unwrap())
        .expect("Failed to load config file");
    let shutdown_key = app_config.get_security().shutdown_key.clone();
    if body.key == shutdown_key {
        let immediate = body.immediate;
        // 执行关机
        execute_shutdown(immediate);

        return (
            StatusCode::OK,
            Json(serde_json::json!({
                "code": 0,
                "msg": "关机指令已执行",
                "success": true,
            })),
        );
    }

    (
        StatusCode::FORBIDDEN,
        Json(serde_json::json!({
            "code": -1,
            "msg": "密钥错误",
            "success": false,
        })),
    )
}
// 处理重启请求
pub async fn reboot_handler(
    Json(body): Json<ShutdownRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let app_dir = get_app_dir();
    let app_config = AppConfig::from_file(AppConfig::default_path(&app_dir).to_str().unwrap())
        .expect("Failed to load config file");
    let shutdown_key = app_config.get_security().shutdown_key.clone();
    if body.key == shutdown_key {
        let immediate = body.immediate;
        // 执行关机
        execute_reboot(immediate);

        return (
            StatusCode::OK,
            Json(serde_json::json!({
                "code": 0,
                "msg": "重启指令已执行",
                "success":true
            })),
        );
    }

    (
        StatusCode::FORBIDDEN,
        Json(serde_json::json!({
            "code": -1,
            "msg": "密钥错误",
            "success":false
        })),
    )
}

pub async fn get_device_status() -> (StatusCode, Json<serde_json::Value>) {
    println!("get_device_status");
    return (
        StatusCode::OK,
        Json(serde_json::json!({
            "code": 0,
            "msg": "状态已发送",
            "success":true,
            "data":{}
        })),
    );
}

pub async fn get_system_info() -> (StatusCode, Json<serde_json::Value>) {
    println!("get_system_info_json");
    let system_info = get_system_info_json();
    let data_json = match system_info {
        Some(info) => serde_json::to_value(info).unwrap_or(serde_json::Value::Null),
        None => serde_json::Value::Object(serde_json::Map::new()), // 返回空对象 {}
    };
    return (
        StatusCode::OK,
        Json(serde_json::json!({
            "code": 0,
            "msg": "状态已发送",
            "success":true,
            "data": data_json
        })),
    );
}
