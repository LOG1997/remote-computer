use crate::system_control::check::launch_app_safe;
use std::process::Command;

pub fn execute_shutdown(immediate: bool) {
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
                }
            }
            Err(e) => {
                eprintln!("执行 关机命令 出错: {}", e);
            }
        }
    }
}

pub fn execute_reboot(immediate: bool) {
    println!("正在执行重启指令...");
    #[cfg(target_os = "windows")]
    {
        // Windows: shutdown /r /t 0 (立即重启)
        // 注意：Windows下通常不需要 sudo，但需要管理员权限运行此 Rust 程序
        match Command::new("shutdown")
            .arg("/r")
            .arg("/t")
            .arg(match immediate {
                true => "0",
                false => "60",
            })
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    println!("Windows 重启指令发送成功");
                } else {
                    eprintln!(
                        "Windows 重启失败: {:?}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
            Err(e) => eprintln!("执行 Windows 重启命令出错: {}", e),
        }
    }
    #[cfg(target_os = "linux")]
    {
        // Linux 尝试多种方案
        println!("正在尝试 Linux 重启...");
        // 方案 1: 使用 systemctl (现代大多数 Linux 发行版推荐，且如果服务以 root 运行则无需 sudo)
        // 尝试直接执行 reboot (如果程序以 root 运行)
        let status = Command::new("shutdown")
            .arg("-r")
            .arg("-h")
            .arg(match immediate {
                true => "now",
                false => "+1",
            })
            .status();

        match status {
            Ok(status) => {
                if status.success() {
                    println!("Rebooting...");
                } else {
                    println!("Failed to reboot");
                }
            }
            Err(err) => {
                println!("Failed to reboot: {}", err);
            }
        }
    }
}

pub fn launch_app(app_name: &str) -> Result<(), String> {
    launch_app_safe(app_name)?;
    Ok(())
}

// 使用自研的to命令来启动应用
pub fn launch_app_with_to(app_name: &str) -> Result<(), String> {
    let command = format!("to {}", app_name);
    #[cfg(target_os = "windows")]
    {
        match Command::new("cmd").arg("/c").arg(command).output() {
            Ok(output) => {
                if output.status.success() {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                } else {
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(err) => {
                eprintln!("Failed to execute command: {}", err);
            }
        }
    }
    #[cfg(target_os = "linux")]
    {
        match Command::new("/bin/sh").arg("-c").arg(command).output() {
            Ok(output) => {
                if output.status.success() {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                } else {
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(err) => {
                eprintln!("Failed to execute command: {}", err);
            }
        }
    }
    #[cfg(target_os = "macos")]
    {
        match Command::new("to").arg("-a").arg(app_name).output() {
            Ok(output) => {
                if output.status.success() {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                } else {
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(err) => {
                eprintln!("Failed to execute command: {}", err);
            }
        }
    }

    Ok(())
}
