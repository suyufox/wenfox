// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::{Parser, Subcommand};
use colored::Colorize;

mod app;
mod cli;
mod configs;
mod server;

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
        port: Option<u16>,
    },
    /// 显示版本信息
    Version,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { port } => {
            println!("🚀 启动 Wenfox 应用 {}", "[DEBUG]".yellow());
            if let Some(p) = port {
                println!("📡 使用端口: {}", p.to_string().green());
            }
            run();
            Ok(())
        }
        Commands::Version => {
            println!(
                "{} {}",
                "Wenfox CLI".bold().green(),
                env!("CARGO_PKG_VERSION").bold()
            );
            Ok(())
        }
    }
}
