// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use colored::*;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn main() {
    let builder = tauri::Builder::default();

    // 通用类插件初始化 | common plugin init
    builder
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        // 日志配置 | log config
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                    file_name: Some("logs".to_string()),
                }))
                .format(move |out, message, record| {
                    let color = match record.level() {
                        log::Level::Error => colored::Color::Red,
                        log::Level::Warn => colored::Color::Yellow,
                        log::Level::Info => colored::Color::Green,
                        log::Level::Debug => colored::Color::Cyan,
                        log::Level::Trace => colored::Color::Magenta,
                    };
                    out.finish(format_args!(
                        "{} {}",
                        format!("[{}]", record.level()).color(color),
                        message
                    ))
                })
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .max_file_size(40_960)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
                .build(),
        )
        // .setup(|wenfox| {
        //     let server = crate::server::WenfoxServer::default();
        //     wenfox.manage(std::sync::Arc::new(tokio::sync::Mutex::new(server)));
        //     let wenfox_handle = wenfox.handle().clone();

        //     tauri::async_runtime::spawn(async move {
        //         let server_instance =
        //             wenfox_handle.state::<std::sync::Arc<tokio::sync::Mutex<crate::server::WenfoxServer>>>();
        //         let mut server = server_instance.lock().await;

        //         if let Err(e) = server.run() {
        //             log::error!("启动服务器失败: {}", e);
        //         }
        //     });

        //     Ok(())
        // })
        // .invoke_handler(tauri::generate_handler![
        //     crate::server::run_server,
        //     crate::server::stop_server,
        //     crate::server::restart_server,
        // ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application | 运行 tauri 应用时发生错误");
}
