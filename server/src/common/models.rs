use anyhow::Result;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::{Mutex, mpsc::UnboundedSender, oneshot};
use volumecontrol::AudioDevice;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug)]
pub enum AudioCommand {
    SetVolume {
        volume: u8,
        reply: oneshot::Sender<u8>, // 成功返回实际音量，失败返回错误信息
    },
    GetVolume {
        reply: oneshot::Sender<u8>,
    },
}
#[derive(Debug, Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub audio_tx: UnboundedSender<AudioCommand>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MsgType {
    LaunchApp,
    SystemControl,
    GetSystemInfo,
    GetVolume,
    SetVolume,
    BrowserControl,
    Ping,
    Error,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ParamValue {
    String(String),
    Bool(bool),
    U64(u64),
    // 如果需要支持浮点数，可以加 F64(f64)
}
#[derive(Debug, Clone, Deserialize, Serialize)]
/// 命令主体格式
pub struct CommandType {
    /// 命令类型
    pub command_type: String,
    pub param: Option<Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// 限制websocket由客户端到服务的消息格式
pub struct MsgReqModel {
    /// 消息类型
    pub topic: MsgType,
    /// 用于验证这条消息的真实性，用于服务验证
    pub token: String,
    /// 命令，以string形式传递，方便读取，有参数以空格分隔开，取Option类型，是因为ping消息可能不需要带command
    pub command: Option<CommandType>,
    /// 发送时间，以字符串形式表示
    pub date_time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MsgRspModel<T> {
    /// 消息类型，与req里面的消息类型相同
    pub topic: MsgType,
    /// 用于验证这条消息的真实性，用于客户端验证
    pub token: String,
    /// 返回数据，以json形式表示
    pub data: Option<T>,
    /// 状态码
    pub code: i32,
    /// 报错信息，执行错误返回错误信息，没错就为none
    pub msg: Option<String>,
    /// 是否执行成功
    pub success: bool,
    /// 发送时间，以字符串形式表示
    pub date_time: DateTime<Utc>,
}

impl<T> MsgRspModel<T> {
    pub fn success(topic: MsgType, data: T, msg: Option<String>) -> Self {
        Self {
            topic,
            token: String::new(),
            data: Some(data),
            code: 0,
            msg,
            success: true,
            date_time: DateTime::default(),
        }
    }
    pub fn error(topic: MsgType, msg: Option<String>) -> Self {
        Self {
            topic,
            token: String::new(),
            data: None,
            code: -1,
            msg,
            success: false,
            date_time: DateTime::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub web_server: WebServerConfig,
    pub security: SecurityConfig,
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

impl AppConfig {
    /// 从指定路径加载配置文件
    pub fn from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: AppConfig = toml::from_str(&content)?;
        Ok(config)
    }

    // 获取server配置
    pub fn get_server(&self) -> &WebServerConfig {
        &self.web_server
    }
    // 获取security配置也就是密码
    pub fn get_security(&self) -> &SecurityConfig {
        &self.security
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct QueryAuth {
    token: String,
}
