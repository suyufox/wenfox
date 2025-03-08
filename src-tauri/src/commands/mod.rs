use std::sync::Arc;
use wenfox_server::WenfoxServer;
use tauri::async_runtime::Mutex;

pub struct ServerState {
    pub server: Arc<Mutex<Option<WenfoxServer>>>,
}

impl Default for ServerState {
    fn default() -> Self {
        Self {
            server: Arc::new(Mutex::new(None)),
        }
    }
}

#[tauri::command]
pub async fn start_server(port: u16, state: tauri::State<'_, ServerState>) -> Result<(), String> {
    let config = WenfoxServer::new().with_port(port).with_host("0.0.0.0");

    let mut handle = state.server.lock().await;
    *handle = Some(config);

    // 使用 Tauri 的异步运行时
    let server = handle.take();
    tauri::async_runtime::spawn(async move {
        if let Some(server) = server {
            server.run()
                .map_err(|e| format!("服务器启动失败: {}", e))
        } else {
            Ok(())
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn stop_server(state: tauri::State<'_, ServerState>) -> Result<(), String> {
    // 修正异步锁使用方式
    let mut handle = state.server.lock().await;
    *handle = None;
    Ok(())
}
