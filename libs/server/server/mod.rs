use log::{error, info};
use std::{net::SocketAddr, sync::Arc};
use tauri::{AppHandle, Manager};
use tokio::runtime::Runtime;
use warp::Filter;

#[derive(Clone)]
pub struct ServerState {
    pub app_handle: AppHandle,
    pub log_path: String,
}

pub struct WebServer {
    runtime: Runtime,
    addr: SocketAddr,
}

impl WebServer {
    pub fn start(config: ServerConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;

        let addr = format!("0.0.0.0:{}", config.port).parse()?;
        let state = config.state.clone();

        // 初始化独立日志文件
        let file_log = tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::File {
            path: std::path::PathBuf::from(&state.log_path),
        });

        runtime.spawn(async {
            info!(target: "server", "Starting web server on {}", addr);

            let routes = warp::any()
                .and(warp::path("health"))
                .map(|| "OK")
                .with(warp::log::custom(|info| {
                    log::info!("{} {} {}", info.method(), info.path(), info.status());
                }));

            warp::serve(routes).run(addr).await;
        });

        Ok(WebServer { runtime, addr })
    }

    pub fn stop(self) {
        self.runtime.shutdown_background();
        info!(target: "server", "Server stopped");
    }
}

pub struct ServerConfig {
    pub port: u16,
    pub state: Arc<ServerState>,
}

// Tauri 集成
pub fn init_server(app: &AppHandle, port: u16) -> Result<(), String> {
    let state = Arc::new(ServerState {
        app_handle: app.clone(),
        log_path: app
            .path_resolver()
            .app_log_dir()
            .expect("Failed to get log dir")
            .join("server.log")
            .to_string_lossy()
            .into_owned(),
    });

    let config = ServerConfig { port, state };

    WebServer::start(config)
        .map(|server| {
            app.manage(server);
            Ok(())
        })
        .map_err(|e| format!("Failed to start server: {}", e))
}
