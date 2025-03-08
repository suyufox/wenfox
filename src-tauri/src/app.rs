use colored::*;
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tauri::async_runtime::Mutex;

use crate::commands::ServerState;

// Tauri app 主函数 | tauri app main function
// 通过该函数启动应用 | start the app by this function
// 该函数会在应用启动时被调用 | this function will be called when the app starts
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn wenfox_app_main() {
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|wenfox, argv, _cwd| {
            println!("a new app instance was opened with {argv:?} and the deep link event was already triggered");
            println!("一个新的应用实例被打开了，已经触发了深度链接事件");
            // when defining deep link schemes at runtime, you must also check `argv` here
            let _ = show_window(wenfox);
        }));
    }

    // 通用类插件初始化 | common plugin init
    builder
        .plugin(tauri_plugin_deep_link::init())
        // https://github.com/HuakunShen/tauri-plugin-system-info
        // 第三方tauri插件 | third-party tauri plugin
        // 用于获取系统相关信息 | for getting system related information
        .plugin(tauri_plugin_system_info::init())
        // https://github.com/ayangweb/tauri-plugin-fs-pro
        //  .plugin(tauri_plugin_fs_pro::init())
        // https://github.com/HuakunShen/tauri-plugin-network
        //.plugin(tauri_plugin_network::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
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
        // 初始化加密配置
        .setup(|wenfox| {
            let salt_path = wenfox
                .path()
                .app_local_data_dir()
                .expect("could not resolve app local data path | 无法解析应用本地数据路径")
                .join("salt.txt");
            wenfox
                .handle()
                .plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;
            Ok(())
        })
        // 调试模式下的日志配置
        .setup(|wenfox| {
            if cfg!(debug_assertions) {
                use colored::*;
                use log::Level;

                tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout).filter(|metadata| {
                    let level_str = match metadata.level() {
                        Level::Error => format!("[{}]", metadata.level()).red(),
                        Level::Warn => format!("[{}]", metadata.level()).yellow(),
                        Level::Info => format!("[{}]", metadata.level()).green(),
                        Level::Debug => format!("[{}]", metadata.level()).cyan(),
                        Level::Trace => format!("[{}]", metadata.level()).magenta(),
                    };
                    println!("{} {}", level_str, metadata.target().blue());
                    true
                });

                wenfox.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .format(move |out, message, record| {
                            let color = match record.level() {
                                log::Level::Error => colored::Color::Red,
                                log::Level::Warn => colored::Color::Yellow,
                                log::Level::Info => colored::Color::Green,
                                log::Level::Debug => colored::Color::Cyan,
                                log::Level::Trace => colored::Color::Magenta,
                            };
                            out.finish(format_args!(
                                // 修改方法名 message -> log
                                "{} {}",
                                format!("[{}]", record.level()).color(color),
                                message
                            ))
                        })
                        .build(),
                )?;
            }
            Ok(())
        })
        // 初始化任务管理器
        // .setup(|wenfox| {
        //     let quest_manager = Arc::new(Mutex::new(QuestManager::new()));
        //     init_quest_module(&quest_manager, wenfox);
        //     Ok(())
        // })
        // 初始化更新插件
        .setup(|wenfox| {
            let _ = wenfox
                .handle()
                .plugin(tauri_plugin_updater::Builder::new().build());
            Ok(())
        })
        .manage(ServerState {
            server: Arc::new(Mutex::new(None)),
        })
        .invoke_handler(tauri::generate_handler![
            crate::commands::start_server,
            crate::commands::stop_server,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application | 运行 tauri 应用时发生错误");
}

// 显示窗口
// 仅在桌面平台上使用 | only use on desktop platform
fn show_window(wenfox: &AppHandle) {
    let windows = wenfox.webview_windows();
    windows
        .values()
        .next()
        .expect("Sorry, no window found | 抱歉，没有找到窗口")
        .set_focus()
        .expect("Can't Bring Window to Focus | 无法将窗口置于焦点");
}
