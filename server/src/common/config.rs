// src/config.rs
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub web_server: WebServerConfig,
    pub security: SecurityConfig,
    pub mqtt: MqttConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WebServerConfig {
    pub enable: bool,
    pub host: String,
    pub port: u16,
    pub https: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SecurityConfig {
    pub shutdown_key: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MqttConfig {
    pub enable: bool,
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub client_id: String,
    pub device_name: String,
    pub device_model: String,
    pub tls: bool,
    pub interval: u64,
    pub expire_time: u64,
    pub launch_app: serde_json::Value,
}

impl AppConfig {
    /// 从指定路径加载配置文件
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: AppConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// 获取默认的配置文件路径（例如在 app_dir 下）
    pub fn default_path(app_dir: &PathBuf) -> PathBuf {
        app_dir.join("config.toml")
    }
    // 获取server
    pub fn get_server(&self) -> &WebServerConfig {
        &self.web_server
    }
    // 获取security
    pub fn get_security(&self) -> &SecurityConfig {
        &self.security
    }
    // 获取mqtt配置
    pub fn get_mqtt(&self) -> &MqttConfig {
        &self.mqtt
    }
}
