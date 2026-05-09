mod common;
use common::app::get_app_dir;
use common::config::AppConfig;

mod api;
mod system;

mod app;
use app::mqtt_ha::start_mqtt;
use app::web_server::start_web_server;

use tokio::signal;
use tokio::task::JoinHandle;

// 关机主函数
#[tokio::main]
async fn main() {
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
            // 注意：这里需要确保 start_web_server 接受所有权或者引用生命周期足够
            // 如果 start_web_server 签名是 &WebServerConfig，clone 后传入引用是安全的
            start_web_server(&config).await;
        });
        handles.push(handle);
    }
    if enable_mqtt {
        println!("启动MQTT服务...");
        let config = mqtt_config.clone();
        // 同样将 MQTT 任务放入后台
        let handle = tokio::spawn(async move {
            start_mqtt(&config).await;
        });
        handles.push(handle)
    }

    if handles.is_empty() {
        println!("没有启用任何服务，程序退出。");
        return;
    }

    println!("所有服务已启动，按 Ctrl+C 退出...");

    if let Err(e) = signal::ctrl_c().await {
        eprintln!("未能监听 Ctrl+C 信号: {}", e);
    }

    println!("\n收到退出信号，正在关闭服务...");

    // 4. 优雅关闭：中止所有后台任务
    for handle in handles {
        handle.abort();
    }

    println!("程序已退出。");
}
