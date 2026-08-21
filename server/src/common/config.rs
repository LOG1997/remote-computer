use std::{env, fs, path::PathBuf};

pub fn get_root_dir() -> PathBuf {
    env::current_dir().expect("无法获取根目录")
}

pub fn get_init_config() -> PathBuf {
    let cargo_name = env!("CARGO_CRATE_NAME");
    let dev_mode = env::var("DEV_MODE")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);
    // 开发模式直接读取本地配置环境地址
    let dev_config_path = env::current_dir();
    // if dev_mode {
    return dev_config_path
        .expect("开发环境不能获取你的配置文件地址")
        .join("config.toml");
    // } else {
    //     // release模式
    //     let user_home_path = home::home_dir();
    //     let config_path = match user_home_path {
    //         Some(value) => value.join(".config").join(cargo_name).join("config.toml"),
    //         None => env::current_exe().expect("无法获取文件夹位置，请确认您的权限"),
    //     };
    //     if !config_path.exists() {
    //         fs::create_dir_all(config_path.parent().expect("创建配置文件失败"))
    //             .expect("创建配置文件失败");
    //         fs::File::create(&config_path).expect("创建配置文件失败");
    //     }
    //     return config_path;
    // }
}
