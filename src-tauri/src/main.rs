// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(name = "wenfox-cli")]
#[command(version = "0.1.0")]
#[command(about = "Wenfox 应用命令行接口")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 启动桌面应用
    Run {
        /// 指定监听端口
        #[arg(short, long)]
        port: Option<u16>
    },
    /// 显示版本信息
    Version
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
  let cli = Cli::parse();
    
    match cli.command {
        Commands::Run { port } => {
            println!("🚀 启动 Wenfox 应用 {}", 
                "[DEBUG]".yellow());
            if let Some(p) = port {
                println!("📡 使用端口: {}", p.to_string().green());
            }
            run();
            Ok(())
        }
        Commands::Version => {
            println!("{} {}", 
                "Wenfox CLI".bold().green(),
                env!("CARGO_PKG_VERSION").bold());
            Ok(())
        }
    }
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        use colored::*;
        use log::Level;
        use tauri_plugin_log::TargetKind;

        tauri_plugin_log::Target::new(TargetKind::Stdout)
         .filter(|metadata| {
           let level_str = match metadata.level() {
             Level::Error => format!("[{}]", metadata.level()).red(),
             Level::Warn => format!("[{}]", metadata.level()).yellow(),
             Level::Info => format!("[{}]", metadata.level()).green(),
             Level::Debug => format!("[{}]", metadata.level()).cyan(),
             Level::Trace => format!("[{}]", metadata.level()).magenta()
           };
           println!("{} {}", level_str, metadata.target().blue());
           true
         });

        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .format(move |out, message, record| {
              let color = match record.level() {
                log::Level::Error => colored::Color::Red,
                log::Level::Warn => colored::Color::Yellow,
                log::Level::Info => colored::Color::Green,
                log::Level::Debug => colored::Color::Cyan,
                log::Level::Trace => colored::Color::Magenta
            };
            out.finish(format_args!(  // 修改方法名 message -> log
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
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
