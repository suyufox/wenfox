use serde::{de::DeserializeOwned, Serialize};
use std::{fmt, path::PathBuf};
use tauri::{AppHandle, Manager};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),
    #[error("解析错误: {0}")]
    Parse(String),
    #[error("未知文件格式")]
    UnknownFormat,
}

impl<T: fmt::Display> From<serde_yaml::Error> for ConfigError {
    fn from(err: serde_yaml::Error) -> Self {
        ConfigError::Parse(format!("YAML解析错误: {}", err))
    }
}

impl<T: fmt::Display> From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> Self {
        ConfigError::Parse(format!("JSON解析错误: {}", err))
    }
}

// 获取应用配置目录
pub fn get_config_dir(app: &AppHandle) -> PathBuf {
    app.path().app_config_dir().expect("无法获取配置目录")
}

// 通用配置文件加载
pub fn load_config<T: DeserializeOwned>(app: &AppHandle, filename: &str, format: &str) -> Result<T, ConfigError> {
    let config_dir = get_config_dir(app);
    std::fs::create_dir_all(&config_dir)?;

    let path = config_dir.join(filename);
    let content = std::fs::read_to_string(&path)?;

    match format.to_lowercase().as_str() {
        "yaml" | "yml" => serde_yaml::from_str(&content).map_err(Into::into),
        "json" => serde_json::from_str(&content).map_err(Into::into),
        "toml" => toml::from_str(&content).map_err(|e| ConfigError::Parse(e.to_string())),
        _ => Err(ConfigError::UnknownFormat),
    }
}

// 通用配置文件保存
pub fn save_config<T: Serialize>(app: &AppHandle, data: &T, filename: &str, format: &str) -> Result<(), ConfigError> {
    let config_dir = get_config_dir(app);
    std::fs::create_dir_all(&config_dir)?;

    let path = config_dir.join(filename);
    let content = match format.to_lowercase().as_str() {
        "yaml" | "yml" => serde_yaml::to_string(data)?,
        "json" => serde_json::to_string_pretty(data)?,
        "toml" => toml::to_string(data)?,
        _ => return Err(ConfigError::UnknownFormat),
    };

    std::fs::write(path, content)?;
    Ok(())
}

// 自动检测文件格式
pub fn detect_format(filename: &str) -> Option<&str> {
    filename
        .rsplit('.')
        .next()
        .and_then(|ext| match ext.to_lowercase().as_str() {
            "yml" | "yaml" => Some("yaml"),
            "json" => Some("json"),
            "toml" => Some("toml"),
            _ => None,
        })
}

// 自动加载配置（带格式检测）
pub fn auto_load_config<T: DeserializeOwned>(app: &AppHandle, filename: &str) -> Result<T, ConfigError> {
    let format = detect_format(filename).ok_or(ConfigError::UnknownFormat)?;

    load_config(app, filename, format)
}

// 使用示例
// 加载配置
// let app = ...; // 获取 AppHandle
// let config: MyConfig = configs::auto_load_config(&app, "settings.yaml")?;

// // 保存配置
// configs::save_config(&app, &config, "settings.toml", "toml")?;
