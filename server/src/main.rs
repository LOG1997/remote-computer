use axum::{
    Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    serve,
};
use axum_server::tls_rustls::RustlsConfig;
use http::header;
use serde::Deserialize;
use tower_http::services::ServeDir; // 用于设置 Content-Type

mod common;
use common::app::get_app_dir;
use common::config::AppConfig;

mod api;
mod system;
use api::handle::{get_device_status, get_system_info, reboot_handler, shutdown_handler};

// 关机主函数
#[tokio::main]
async fn main() {
    let is_dev = cfg!(debug_assertions);
    let app_dir = get_app_dir();
    let app_config = AppConfig::from_file(AppConfig::default_path(&app_dir).to_str().unwrap())
        .expect("Failed to load config file");
    let security_config = app_config.get_security();
    let server_config = app_config.get_server();
    let shutdown_key = security_config.shutdown_key.clone();
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
    println!("🔑 密钥: {}", shutdown_key);

    let static_files_root = if is_dev {
        "../client/apps/web/dist".into()
    } else {
        app_dir.join("web") // 生产环境：/path/to/bin/web/
    };

    let static_files_service = ServeDir::new(static_files_root);

    // 路由配置
    let app = Router::new()
        .route("/", get(serve_index_html))
        .fallback_service(static_files_service)
        .route("/shutdown", post(shutdown_handler))
        .route("/getStatus", post(get_device_status))
        .route("/getDeviceInfo", post(get_system_info))
        .route("/reboot", post(reboot_handler))
        // 允许跨域 → 网页必须
        .layer(tower_http::cors::CorsLayer::permissive());
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    // 启动服务
    // 根据is_https 判断启动http或https服务
    if is_https {
        println!("启动服务: https://{}", addr);
        axum_server::bind_rustls(addr, tls_config)
            .serve(app.into_make_service())
            .await
            .unwrap();
    } else {
        println!("启动服务: http://{}", addr);
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }

    // axum_server::bind_rustls(addr, tls_config)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}
async fn serve_index_html() -> impl IntoResponse {
    let is_dev = cfg!(debug_assertions);

    let path = if is_dev {
        "../client/apps/web/dist/index.html".to_string()
    } else {
        // 生产环境：基于二进制文件目录构建 index.html 路径
        let app_dir = get_app_dir();
        app_dir
            .join("web")
            .join("index.html")
            .to_str()
            .unwrap()
            .to_string()
    };
    println!("Reading index.html from {}", path);
    match tokio::fs::read_to_string(path).await {
        Ok(content) => {
            // 返回 HTML 内容，并设置正确的 Content-Type
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
                content,
            )
        }
        Err(e) => {
            eprintln!("❌ 无法读取 index.html: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(header::CONTENT_TYPE, "text/plain")],
                "Internal Server Error: Index file not found".to_string(),
            )
        }
    }
}
