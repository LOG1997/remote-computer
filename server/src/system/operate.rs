use serde::{Deserialize, Serialize};
use serde_json;
use std::process::Command;
use sysinfo::{Components, CpuRefreshKind, Disks, Networks, System};

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
        let status = Command::new("sudo")
            .arg("/sbin/shutdown")
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
        let status = Command::new("sudo")
            .arg("/sbin/reboot")
            .arg("-h")
            .arg(match immediate {
                true => "now",
                false => "now",
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
// 定义返回数据的结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct SystemInfoResponse {
    pub os: OsInfo,
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub disks: Vec<DiskInfo>,
    pub components: Vec<ComponentInfo>, // 通常包含显卡温度等信息，取决于硬件监控支持
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OsInfo {
    pub name: Option<String>,
    pub kernel_version: Option<String>,
    pub os_version: Option<String>,
    pub host_name: Option<String>,
    pub platform: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CpuInfo {
    pub physical_core_count: usize,
    pub total_core_count: usize,
    pub brand: String,
    pub frequency: u64, // MHz
    pub usage: f32,     // 整体 CPU 使用率 %
    pub temperature: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemoryInfo {
    pub total_memory: u64, // bytes
    pub used_memory: u64,  // bytes
    pub total_swap: u64,   // bytes
    pub used_swap: u64,    // bytes
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub file_system: String,
    pub total_space: u64,     // bytes
    pub available_space: u64, // bytes
    pub is_removable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ComponentInfo {
    pub label: String,
    pub temperature: f32,
    pub max_temperature: f32,
    pub critical_threshold: Option<f32>,
}
/// 获取系统信息并返回 JSON 字符串
pub fn get_system_info_json() -> Option<SystemInfoResponse> {
    let mut sys = System::new_all();
    // 刷新所有数据
    sys.refresh_all();

    // 专门刷新 CPU 使用率，因为 new_all 可能不包含即时使用率，或者需要单独刷新
    sys.refresh_cpu_specifics(CpuRefreshKind::everything());

    // 1. 操作系统信息
    let os_info = OsInfo {
        name: System::name(),
        kernel_version: System::kernel_version(),
        os_version: System::os_version(),
        host_name: System::host_name(),
        platform: System::long_os_version(),
    };
    // 2. CPU 信息
    let cpus = sys.cpus();
    let global_cpu_usage = sys.global_cpu_usage();
    let first_cpu = cpus.first();

    let mut cpu_info = CpuInfo {
        physical_core_count: System::physical_core_count().unwrap_or(0),
        total_core_count: cpus.len(),
        brand: first_cpu.map(|c| c.brand().to_string()).unwrap_or_default(),
        frequency: first_cpu.map(|c| c.frequency()).unwrap_or(0),
        usage: global_cpu_usage,
        temperature: 0.0,
    };

    // 3. 内存信息
    let memory_info = MemoryInfo {
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        total_swap: sys.total_swap(),
        used_swap: sys.used_swap(),
    };

    // 4. 硬盘信息
    let disks = Disks::new_with_refreshed_list();
    let disk_infos: Vec<DiskInfo> = disks
        .iter()
        .map(|disk| DiskInfo {
            name: disk.name().to_string_lossy().into_owned(),
            mount_point: disk.mount_point().to_string_lossy().into_owned(),
            file_system: disk.file_system().to_string_lossy().into_owned(),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
            is_removable: disk.is_removable(),
        })
        .collect();

    // 5. 组件信息 (温度/显卡等)
    // 注意：sysinfo 对 GPU 的支持有限，通常通过 Components 获取温度传感器数据
    let components = Components::new_with_refreshed_list();
    let component_infos: Vec<ComponentInfo> = components
        .iter()
        .map(|comp| ComponentInfo {
            label: comp.label().to_string(),
            temperature: comp.temperature().unwrap_or(0.0),
            max_temperature: comp.max().unwrap_or(0.0),
            critical_threshold: comp.critical(),
        })
        .collect();
    // 计算CPU平均温度
    let cpu_comp: Vec<&sysinfo::Component> = components
        .iter()
        .filter(|comp| comp.label().contains("Core"))
        .collect();
    let cpu_ava_temp = cpu_comp
        .iter()
        .map(|comp| comp.temperature().unwrap_or(0.0))
        .sum::<f32>()
        / cpu_comp.len() as f32;
    println!("CPU平均温度: {}°C", cpu_ava_temp);
    cpu_info.temperature = cpu_ava_temp;
    // 组装最终结构
    let system_info = SystemInfoResponse {
        os: os_info,
        cpu: cpu_info,
        memory: memory_info,
        disks: disk_infos,
        components: component_infos,
    };
    Some(system_info)
    // 序列化为 JSON
    // serde_json::to_string_pretty(&system_info)
}
