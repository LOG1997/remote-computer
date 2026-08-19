use serde::{Deserialize, Serialize};
use sysinfo::{Components, CpuRefreshKind, Disks, System};
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
    pub total_memory: u64,     // bytes
    pub used_memory: u64,      // bytes
    pub available_memory: u64, // bytes
    pub total_swap: u64,       // bytes
    pub used_swap: u64,        // bytes
    pub usage: f32,
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

    // 0. 操作系统信息
    let os_info = OsInfo {
        name: System::name(),
        kernel_version: System::kernel_version(),
        os_version: System::os_version(),
        host_name: System::host_name(),
        platform: System::long_os_version(),
    };
    // 1. CPU 信息
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

    // 2. 内存信息
    let mut memory_info = MemoryInfo {
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        available_memory: sys.available_memory(),
        total_swap: sys.total_swap(),
        used_swap: sys.used_swap(),
        usage: 0.0,
    };

    // 3. 硬盘信息
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

    // 4. 组件信息 (温度/显卡等)
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
    cpu_info.temperature = cpu_ava_temp;
    // 计算内存使用率
    let memory_usage: f32 = (100 * memory_info.used_memory / memory_info.total_memory) as f32;
    memory_info.usage = memory_usage;
    // 组装最终结构
    let system_info = SystemInfoResponse {
        os: os_info,
        cpu: cpu_info,
        memory: memory_info,
        disks: disk_infos,
        components: component_infos,
    };
    Some(system_info)
}
