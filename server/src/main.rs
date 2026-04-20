use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use axum_server::tls_rustls::RustlsConfig;
use http::header;
use serde::Deserialize;
use std::path::PathBuf;
use std::process::Command;
use tower_http::services::ServeDir; // 用于设置 Content-Type

// 配置
const SHUTDOWN_KEY: &str = "xinglugu";
const PORT: u16 = 52011;

// 接收前端传的密钥
#[derive(Deserialize)]
struct ShutdownRequest {
    key: String,
    immediate: bool,
}

fn get_app_dir() -> PathBuf {
    // current_exe() 获取当前运行的二进制文件的完整路径
    // parent() 获取其所在目录
    std::env::current_exe()
        .expect("Failed to get current executable path")
        .parent()
        .expect("Failed to get parent directory of executable")
        .to_path_buf()
}

// 关机主函数
#[tokio::main]
async fn main() {
    let is_dev = cfg!(debug_assertions);
    let app_dir = get_app_dir();
    let mode = if is_dev {
        "dev".to_string()
    } else {
        "production".to_string()
    };
    // 1. 加载自签名证书
    let cert_path = if mode == "dev" {
        "cert.pem".into() // 开发环境证书路径，请根据实际情况修改
    } else {
        app_dir.join("cert.pem")
    };

    let key_path = if mode == "dev" {
        "key.pem".into() // 开发环境私钥路径，请根据实际情况修改
    } else {
        app_dir.join("key.pem") // 生产环境私钥路径
    };

    // 3. 加载自签名证书
    let tls_config =
        RustlsConfig::from_pem_file(cert_path.to_str().unwrap(), key_path.to_str().unwrap())
            .await
            .unwrap();

    println!("✅ 远程关机服务已启动");
    println!("📡 端口: {}", PORT);
    println!("🔑 密钥: {}", SHUTDOWN_KEY);

    let static_files_root = if is_dev {
        "../client/apps/web/web".into()
    } else {
        app_dir.join("web") // 生产环境：/path/to/bin/web/
    };

    let static_files_service = ServeDir::new(static_files_root);

    // 路由配置
    let app = Router::new()
        .route("/", get(serve_index_html))
        .fallback_service(static_files_service)
        .route("/shutdown", post(shutdown_handler))
        // 允许跨域 → 网页必须
        .layer(tower_http::cors::CorsLayer::permissive());

    // 启动服务
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], PORT));
    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
async fn serve_index_html() -> impl IntoResponse {
    let is_dev = cfg!(debug_assertions);

    let path = if is_dev {
        "../client/apps/web/web/index.html".to_string()
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
// 处理关机请求
async fn shutdown_handler(
    Json(body): Json<ShutdownRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    if body.key == SHUTDOWN_KEY {
        let immediate = body.immediate;
        // 执行关机
        execute_shutdown(immediate);

        return (
            StatusCode::OK,
            Json(serde_json::json!({
                "code": 0,
                "msg": "关机指令已执行"
            })),
        );
    }

    (
        StatusCode::FORBIDDEN,
        Json(serde_json::json!({
            "code": -1,
            "msg": "密钥错误"
        })),
    )
}

// 跨平台关机
fn execute_shutdown(immediate: bool) {
    println!("正在执行关机指令...");

    #[cfg(target_os = "windows")]
    {
        // Windows: shutdown /s /t 0 (立即关机)
        // 注意：Windows下通常不需要 sudo，但需要管理员权限运行此 Rust 程序
        match Command::new("shutdown")
            .arg("/s")
            .arg("/t")
            .arg(match immediate {
                true => "0",
                false => "60",
            })
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    println!("Windows 关机指令发送成功");
                } else {
                    eprintln!(
                        "Windows 关机失败: {:?}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
            Err(e) => eprintln!("执行 Windows 关机命令出错: {}", e),
        }
    }

    #[cfg(target_os = "linux")]
    {
        // Linux 尝试多种方案
        println!("正在尝试 Linux 关机...");
        // 方案 1: 使用 systemctl (现代大多数 Linux 发行版推荐，且如果服务以 root 运行则无需 sudo)
        // 尝试直接执行 shutdown (如果程序以 root 运行)
        let status = Command::new("shutdown")
            .arg("-h")
            .arg(match immediate {
                true => "now",
                false => "+1",
            }) // +1 表示 1 分钟后关机
            .status();

        match status {
            Ok(exit_status) => {
                if exit_status.success() {
                    println!("关机命令执行成功");
                } else {
                    eprintln!("关机命令执行失败, 退出码: {:?}", exit_status.code());
                    // 如果失败，尝试传统 shutdown
                    fallback_shutdown_linux();
                }
            }
            Err(e) => {
                eprintln!("执行 systemctl 出错: {}, 尝试备用方案", e);
                // fallback_shutdown_linux();
            }
        }
    }
}

#[cfg(target_os = "linux")]
fn fallback_shutdown_linux() {
    println!("尝试备用关机命令: shutdown -h now");
    // 注意：这里去掉 sudo，假设当前运行该 Rust 程序的用户已经有 sudo 免密权限，或者程序本身以 root 运行
    // 如果必须用 sudo，请确保配置了 /etc/sudoers 允许该用户无密码执行 shutdown
    match Command::new("sudo")
        .arg("shutdown")
        .arg("-h")
        .arg("+1")
        .status()
    {
        Ok(exit_status) => {
            if !exit_status.success() {
                eprintln!("备用关机命令失败, 退出码: {:?}", exit_status.code());
            }
        }
        Err(e) => eprintln!("执行备用关机命令出错: {}", e),
    }
}
