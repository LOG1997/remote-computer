use std::process::{Command, Stdio};

pub fn is_app_available(app_name: &str) -> bool {
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("where").arg(app_name).output();
        return output.map(|o| o.status.success()).unwrap_or(false);
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        let output = Command::new("which").arg(app_name).output();
        return output.map(|o| o.status.success()).unwrap_or(false);
    }
}

pub fn launch_app_safe(app_name: &str) -> Result<(), String> {
    if !is_app_available(app_name) {
        return Err(format!("应用 {} 未找到", app_name));
    }

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
        const DETACHED_PROCESS: u32 = 0x00000008;

        Command::new(app_name)
            .creation_flags(CREATE_NEW_PROCESS_GROUP | DETACHED_PROCESS)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("启动失败: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        use std::os::unix::process::CommandExt;
        Command::new("open")
            .arg("-a")
            .arg(app_name)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .process_group(0) // 创建新进程组
            .spawn()
            .map_err(|e| format!("启动失败: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        use std::os::unix::process::CommandExt;

        Command::new(app_name)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .process_group(0) // 关键：创建新进程组，防止随父进程退出
            .spawn()
            .map_err(|e| format!("启动失败: {}", e))?;
    }

    Ok(())
}
