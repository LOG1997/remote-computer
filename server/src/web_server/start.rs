use std::net::SocketAddr;
use std::sync::Arc;
use std::thread;

use crate::common::models::AudioCommand;
use crate::system_control::control_volume::{AudioControl, VolumeControl};
use crate::web_server::start::AudioCommand::{GetVolume, SetVolume};
use crate::{
    WEB_ASSETS,
    common::{
        config::get_root_dir,
        models::{AppConfig, AppState, WebServerConfig},
    },
    web_server::{
        self, browser_service::browser_service_handler, user_service::user_service_handler,
    },
};
use anyhow::Result;
use axum::{
    Router,
    body::Body,
    extract::{Path, Request},
    handler::HandlerWithoutStateExt,
    response::Response,
    routing::any,
};
use http::StatusCode;
use rmqtt::topic;
use tokio::net::TcpListener;
use tokio::sync::mpsc::{Receiver, Sender, UnboundedReceiver, UnboundedSender};
use tokio::sync::{Mutex, mpsc, oneshot};
use tokio::task::JoinHandle;
use tower_http::services::ServeDir;

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
        panic!("static web html files not found:{static_files_root:?} and root is {root_dir:?}");
    }
    let (tx, rx) = mpsc::unbounded_channel::<AudioCommand>();
    let _ = create_volume_control(rx, tx.clone()).await;
    let app_state = AppState {
        config: config.clone(),
        audio_tx: tx,
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
        // .fallback_service(fallback_handler.into_service())
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

async fn fallback_handler(req: Request<Body>) -> Response {
    // 1. 获取请求路径，去除前导 '/'，并忽略查询字符串
    let path = req.uri().path().trim_start_matches('/');

    // 2. 如果路径为空（即根路径），设置默认文件为 index.html
    let path = if path.is_empty() { "index.html" } else { path };

    // 3. 尝试从嵌入的静态文件中获取该文件
    if let Some(file) = WEB_ASSETS.get_file(path) {
        // 文件存在，直接返回内容
        let mime = mime_guess::from_path(path).first_or_octet_stream();
        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", mime.as_ref())
            .body(Body::from(file.contents()))
            .unwrap()
    } else {
        // 4. 文件不存在，尝试返回 index.html（SPA 回退）
        // 注意：如果 index.html 也不存在，则返回 404
        if let Some(index) = WEB_ASSETS.get_file("index.html") {
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(Body::from(index.contents()))
                .unwrap()
        } else {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap()
        }
    }
}

async fn create_volume_control(
    mut rx: UnboundedReceiver<AudioCommand>,
    tx: UnboundedSender<AudioCommand>,
) {
    println!("射盒盒盒2");
    thread::spawn(move || {
        // 在新线程中初始化 COM 库 (如果需要)
        match VolumeControl::new() {
            Ok(mut volume_control) => {
                while let Some(audio_command) = rx.blocking_recv() {
                    match audio_command {
                        AudioCommand::SetVolume { volume, reply } => {
                            if let Err(e) = volume_control.set_volume(volume) {
                                log::error!("设置音量失败: {:?}", e);
                            } else {
                                let new_volume = volume_control.get_volume().unwrap_or(0);
                                let _ = reply.send(new_volume);
                            }
                        }
                        AudioCommand::GetVolume { reply } => {
                            let current_volume = volume_control.get_volume().unwrap_or(0);
                            let _ = reply.send(current_volume);
                        }
                    }
                }
            }
            Err(e) => {
                log::error!("初始化音量控制失败: {:?}", e);
            }
        }
    });
}
