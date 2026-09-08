use anyhow::{Ok, Result};
use std::process::{Command, Stdio};
use tracing::info;
pub fn match_app_name(apps_list: &serde_json::Value, app_name: &str) -> Option<String> {
    if apps_list.is_null() {
        return None;
    }
    apps_list
        .as_object()?
        .get(app_name)?
        .as_str()
        .map(|s| s.to_string()) // 复制为 String
}

pub fn launch_app(app_name: String, params: Vec<String>) -> Result<()> {
    if app_name.starts_with("http") {
        open_web_page(app_name, params)?;
        Ok(())
    } else {
        open_app(app_name)?;
        Ok(())
    }
}

fn open_web_page(url: String, params: Vec<String>) -> Result<()> {
    // 不用区分环境了，webbrowser已经做了
    let result = insert_into_template(url.as_str(), params.join(" ").as_str());
    info!("web url {}", result);
    webbrowser::open(result.as_str())?;
    info!("open url sucsss");
    Ok(())
}

fn open_app(app_name: String) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        opener::open(app_name)?;
        Ok(())
    }
    #[cfg(target_os = "linux")]
    {
        Command::new(app_name)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        Ok(())
    }
    #[cfg(target_os = "macos")]
    {
        todo!("open macos file path");
    }
}
fn insert_into_template(template: &str, input: &str) -> String {
    if template.contains("{}") {
        return template.replace("{}", input);
    } else if input.is_empty() {
        return template.to_string();
    } else {
        return template.to_string() + input;
    }
}
