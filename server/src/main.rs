mod common;
use common::app::get_app_dir;
use common::config::AppConfig;

mod api;
mod system;

mod app;
use app::mqtt_client::start_mqtt;
use app::web_server::start_web_server;

use tokio::signal;
use tokio::task::JoinHandle;

use anyhow::{Ok, Result};

// 关机主函数
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");
    let app_dir = get_app_dir();
    let app_config = AppConfig::from_file(AppConfig::default_path(&app_dir).to_str().unwrap())
        .expect("Failed to load config file");

    // 用于存储所有后台任务的句柄
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    // 获取web服务配置
    let web_server_config = app_config.get_server();
    let enable_web_server = web_server_config.enable;

    // 获取mqtt配置
    let mqtt_config = app_config.get_mqtt();
    let enable_mqtt = mqtt_config.enable;

    if enable_web_server {
        println!("启动web服务...");
        let config = web_server_config.clone();
        // spawn 返回 JoinHandle，将其存入向量
        let handle = tokio::spawn(async move {
            start_web_server(&config).await;
        });
        handles.push(handle);
    }

    if enable_mqtt {
        println!("启动MQTT服务...");
        let config = mqtt_config.clone();
        // 修复：将 MQTT 任务也放入后台管理，以便统一控制生命周期
        let handle = tokio::spawn(async move {
            let _ = start_mqtt(&config).await;
        });
        handles.push(handle);
    }

    if handles.is_empty() {
        println!("没有启用任何服务，程序退出。");
        // 修复：必须返回 Result 类型，而不是 unit type ()
        return Ok(());
    }

    println!("所有服务已启动，按 Ctrl+C 退出...");

    if let Err(e) = signal::ctrl_c().await {
        eprintln!("未能监听 Ctrl+C 信号: {}", e);
    }

    println!("\n收到退出信号，正在关闭服务...");

    // 4. 优雅关闭：中止所有后台任务
    for handle in handles {
        handle.abort();
        // 修复：删除此处错误的 Ok(())，abort() 返回 ()，循环体不需要返回值
    }

    println!("程序已退出。");
    Ok(())
}
