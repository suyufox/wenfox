#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    use tauri::{AppHandle, Manager};

    #[cfg(desktop)]
    {
        builder = builder
         .plugin(tauri_plugin_single_instance::init(|wenfox, argv, _cwd| {
             println!("a new app instance was opened with {argv:?} and the deep link event was already triggered | 一个新的应用实例被打开了，已经触发了深度链接事件");
             // when defining deep link schemes at runtime, you must also check `argv` here
             let _ = show_window(wenfox);
         }));
    }

    builder = builder.plugin(tauri_plugin_deep_link::init());

    builder
        // https://github.com/ayangweb/tauri-plugin-fs-pro
        //  .plugin(tauri_plugin_fs_pro::init())
        // https://github.com/HuakunShen/tauri-plugin-network
        //.plugin(tauri_plugin_network::init())
        // https://github.com/HuakunShen/tauri-plugin-system-info
        .plugin(tauri_plugin_system_info::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("logs".to_string()),
                    },
                ))
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
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .max_file_size(40_960 /* bytes */)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
                .build(),
        )
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
        .setup(|wenfox| {
            let salt_path = wenfox
                .path()
                .app_local_data_dir()
                .expect("could not resolve app local data path | 无法解析应用本地数据路径")
                .join("salt.txt");
            wenfox
                .handle()
                .plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;

            let _ = wenfox
                .handle()
                .plugin(tauri_plugin_updater::Builder::new().build()); // 注册 系统托盘
            Ok(())
        })
        .setup(|wenfox| {
            if cfg!(debug_assertions) {
                use colored::*;
                use log::Level;
                use tauri_plugin_log::TargetKind;

                tauri_plugin_log::Target::new(TargetKind::Stdout).filter(|metadata| {
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
        .setup(|wenfox| {
            #[cfg(desktop)]
            let _ = wenfox
                .handle()
                .plugin(tauri_plugin_updater::Builder::new().build());
            Ok(())
        })
        //.invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application | 运行 tauri 应用时发生错误");
}

fn show_window(wenfox: &AppHandle) {
    let windows = wenfox.webview_windows();
    windows
        .values()
        .next()
        .expect("Sorry, no window found | 抱歉，没有找到窗口")
        .set_focus()
        .expect("Can't Bring Window to Focus | 无法将窗口置于焦点");
}
