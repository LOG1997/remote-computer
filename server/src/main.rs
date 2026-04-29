use axum::{Router, routing::post};
use axum_server::tls_rustls::RustlsConfig;
use tokio::task::JoinHandle;
use tower_http::services::ServeDir; // 用于设置 Content-Type

use rmqtt::{Result, context::ServerContext, net::Builder, server::MqttServer};
use rustls::crypto::ring::default_provider;
use simple_logger::SimpleLogger;

mod common;
use common::app::get_app_dir;
use common::config::AppConfig;

mod api;
mod system;
use api::handle::{get_device_status, get_system_info, reboot_handler, shutdown_handler};

// 关机主函数
#[tokio::main]
async fn main() -> Result<()> {
    default_provider().install_default().unwrap();

    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()?;

    let scx = ServerContext::new().build().await;

    let mqtt_handle: JoinHandle<Result<()>> = tokio::spawn(async move {
        MqttServer::new(scx)
            .listener(
                Builder::new()
                    .name("external/tcp")
                    .laddr(([0, 0, 0, 0], 1883).into())
                    .bind()?
                    .tcp()?,
            )
            .listener(
                Builder::new()
                    .name("internal/tcp")
                    .laddr(([0, 0, 0, 0], 11883).into())
                    .bind()?
                    .tcp()?,
            )
            // .listener(
            //     Builder::new()
            //         .name("external/tls")
            //         .laddr(([0, 0, 0, 0], 8883).into())
            //         .tls_key(Some("./key.pem"))
            //         .tls_cert(Some("./cert.pem"))
            //         .bind()?
            //         .tls()?,
            // )
            .listener(
                Builder::new()
                    .name("external/ws")
                    .laddr(([0, 0, 0, 0], 8889).into())
                    .bind()?
                    .ws()?,
            )
            .build()
            .run()
            .await
    });

    let is_dev = cfg!(debug_assertions);
    let app_dir = get_app_dir();
    let app_config = AppConfig::from_file(AppConfig::default_path(&app_dir).to_str().unwrap())
        .expect("Failed to load config file");
    let server_config = app_config.get_server();
    let port = server_config.port;
    let is_https = server_config.https;
    // 1. 加载自签名证书
    let cert_path = app_dir.join("cert.pem");

    let key_path = app_dir.join("key.pem"); // 生产环境私钥路径

    // 3. 加载自签名证书
    let tls_config =
        RustlsConfig::from_pem_file(cert_path.to_str().unwrap(), key_path.to_str().unwrap())
            .await
            .unwrap();

    println!("✅ 远程关机服务已启动");
    println!("📡 端口: {}", port);

    let static_files_root = if is_dev {
        "../client/apps/web/dist".into()
    } else {
        app_dir.join("web")
    };

    // 如果访问 / (根路径)，ServeDir 默认会尝试查找 index.html (取决于配置，通常需确保存在)
    let static_files_service =
        ServeDir::new(&static_files_root).append_index_html_on_directories(true); // 关键：访问目录时自动返回 index.html

    // 路由配置
    let app = Router::new()
        .route("/shutdown", post(shutdown_handler))
        .route("/reboot", post(reboot_handler))
        .route("/getStatus", post(get_device_status))
        .route("/getDeviceInfo", post(get_system_info))
        .fallback_service(static_files_service)
        // 允许跨域 → 网页必须
        .layer(tower_http::cors::CorsLayer::permissive());
    // 启动服务
    let http_result = if is_https {
        let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
        println!("启动服务: https://{}", addr);
        axum_server::bind_rustls(addr, tls_config)
            .serve(app.into_make_service())
            .await
    } else {
        let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
        println!("启动服务: http://{}", addr);
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await
    };
    mqtt_handle.abort(); // 或者根据业务需求处理

    http_result.map_err(|e| e.into())
    // Ok(())
}
