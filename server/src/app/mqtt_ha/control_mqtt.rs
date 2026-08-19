use crate::system_control::operate::execute_shutdown;

pub fn handle_control_online_status(command: &str) {
    if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&command) {
        if let Some(state) = json_val.get("state").and_then(|v| v.as_str()) {
            match state {
                "OFF" => {
                    println!("⚠️ 收到关机指令，准备执行...");
                    execute_shutdown(true);
                    // 注意：shutdown_system 应该是同步阻塞或独立的异步任务
                    // 如果 shutdown_system 会终止当前进程，直接调用即可
                    // shutdown_system();
                }
                "ON" => {
                    println!("✅ 开机指令/不执行");
                    // 通常 HA 发送 ON 只是为了同步状态，不需要特殊操作
                    // 除非你有“远程唤醒”后的确认逻辑
                }
                _ => println!("未知状态指令: {}", state),
            }
        }
    } else {
        eprintln!("❌ 控制消息格式错误: {}", command);
    }
}
