use std::net::SocketAddr;

use crate::{
    common::{
        config::get_root_dir,
        models::{AppConfig, AppState, WebServerConfig},
    },
    system_control::control_volume::VolumeControl,
    web_server::{
        self, browser_service::browser_service_handler, user_service::user_service_handler,
    },
};
use anyhow::Result;
use axum::{Router, routing::any};
use http::Method;
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

pub async fn start_web_server(config: AppConfig) -> Result<()> {
    let is_dev = cfg!(debug_assertions);
    let root_dir = get_root_dir();

    // 静态html地址
    let static_files_root = if is_dev {
        "../client/apps/web/dist".into()
    } else {
        root_dir.join("web")
    };

    // dev的模式下检查dist目录，否则检查web目录，不存在直接报错
    if !static_files_root.exists() {
        panic!("static web html files not found");
    }

    let app_state = AppState {
        config: config.clone(),
    };

    // 如果访问 / (根路径)，ServeDir 默认会尝试查找 index.html (取决于配置，通常需确保存在)
    let static_files_service =
        ServeDir::new(&static_files_root).append_index_html_on_directories(true); // 关键：访问目录时自动返回 index.html
    let web_server_config = config.get_server();

    // 服务地址
    let server_address = format!("{}:{}", web_server_config.host, web_server_config.port);
    let app = Router::new()
        .route("/user", any(user_service_handler))
        .route("/browser", any(browser_service_handler))
        .fallback_service(static_files_service)
        .with_state(app_state)
        .layer(tower_http::cors::CorsLayer::permissive());
    println!("启动web服务:http://{}", server_address);

    let listener = TcpListener::bind(server_address).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}
