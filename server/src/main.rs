mod common;
use common::models::AppConfig;

mod system_control;
mod web_server;

use dotenvy::dotenv;
use tokio::signal;
use tokio::task::JoinHandle;

use anyhow::{Ok, Result};

use crate::{
    system_control::control_volume::{AudioControl, VolumeControl},
    web_server::start::start_web_server,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");
    // 获取配置文件路径（不存在就创建）
    let config_path = common::config::get_init_config();
    let app_config = AppConfig::from_file(&config_path.to_str().unwrap())?;

    // 用于存储所有后台任务的句柄
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    // 获取web服务配置
    let web_server_config = app_config.get_server();
    let enable_web_server = web_server_config.enable;
    // 安全配置

    if enable_web_server {
        let config = app_config;
        // // spawn 返回 JoinHandle，将其存入向量
        // let handle = tokio::spawn(async move {
        start_web_server(config.clone()).await?;
        //     });
        //     handles.push(handle);
    }

    // if handles.is_empty() {
    //     println!("没有启用任何服务，程序退出。");
    //     // 修复：必须返回 Result 类型，而不是 unit type ()
    //     return Ok(());
    // }

    // println!("所有服务已启动，按 Ctrl+C 退出...");

    // if let Err(e) = signal::ctrl_c().await {
    //     eprintln!("未能监听 Ctrl+C 信号: {}", e);
    // }

    // println!("\n收到退出信号，正在关闭服务...");

    // // 4. 优雅关闭：中止所有后台任务
    // for handle in handles {
    //     handle.abort();
    //     // 修复：删除此处错误的 Ok(())，abort() 返回 ()，循环体不需要返回值
    // }

    // println!("程序已退出。");
    Ok(())
}
