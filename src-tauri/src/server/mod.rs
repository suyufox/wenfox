use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use regex::Regex;
use tauri::Manager;
lazy_static! {
    static ref IP_REGEX: Regex = Regex::new(
        r"^((localhost)|(::1)|(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})|(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}))$"
    ).unwrap();
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello from Wenfox server!")
}

// // 带路径参数的处理器
#[get("/user/{name}")]
async fn user_info(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    HttpResponse::Ok().body(format!("Welcome, {}!", name))
}

pub struct WenfoxServer {
    host: String,
    port: u16,
    server_handle: Option<tokio::task::JoinHandle<()>>, // 改为保存处理句柄
    shutdown_sender: Option<tokio::sync::oneshot::Sender<()>>,
}

impl WenfoxServer {
    // 创建服务器默认配置
    // create server default config
    pub fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 47603,
            server_handle: None,
            shutdown_sender: None,
        }
    }

    // 可选的修改器 用于修改默认的服务器地址
    // 正则匹配 IPv4 Ipv6 地址 以及 localhost
    // optional modifier for modifying the default server address
    // regex match ipv4 ipv6 address and localhost
    pub fn set_host(&mut self, host: &str) -> &mut Self {
        if !IP_REGEX.is_match(host) {
            log::error!("无效的主机地址: {}| invalid host address: {}", host, host);
            log::error!("请检查你的主机地址是否正确| please check your host address");
            return self;
        }

        self.host = host.to_string();
        log::info!("设置主机 | set host: {}", self.host);
        self
    }

    pub fn set_port(&mut self, port: u16) -> &mut Self {
        if port > 65535 || port < 0 {
            log::error!("无效的端口号: {} | invalid port: {}", port, port);
            log::error!("请检查你的端口号是否正确| please check your port");
            log::error!("小提示: 端口号范围为 0-65535 | tip: port range is 0-65535");
            log::info!("现在将使用默认端口 47603 | now use default port 47603");
            return self;
        }

        // 添加端口范围验证
        if port == 0 {
            log::error!("无效的端口号: 0 | invalid port: 0");
            log::error!("请检查你的端口号是否正确| please check your port");
            log::error!("小提示: 端口号范围为 0-65535, 但是并不包括 0 | tip: port range is 0-65535, but not include 0");
            log::info!("现在将使用默认端口 47603 | now use default port 47603");
            return self;
        }

        if 0 < port && port < 1024 {
            log::warn!(
                "端口 {} 是一个系统保留端口 | Port {} is a system reserved port",
                port,
                port
            );
            log::warn!("请使用 1024 以上的端口号 | Please use a port number above 1024");
            log::info!("现在将使用默认端口 47603 | now use default port 47603");
            return self;
        }

        // 添加端口占用检测
        if let Err(e) = std::net::TcpListener::bind((self.host.as_str(), port)) {
            log::error!("端口 {} 已被占用 | Port {} is in use: {}", port, port, e);
            log::info!("现在将使用默认端口 47603 | now use default port 47603");
            return self;
        }

        self.port = port;
        log::info!("设置端口 | set port: {}", self.port);
        self
    }

    // 启动服务器
    // start server
    pub fn run(&mut self) -> std::io::Result<()> {
        let addr = format!("{}:{}", self.host, self.port);
        log::info!("正在启动服务器 | Starting server at {}", addr);

        // 创建关闭信号通道
        let (shutdown_sender, shutdown_receiver) = tokio::sync::oneshot::channel();

        self.server_handle = Some(tokio::spawn(async move {
            // 在 Tokio 运行时中创建 Actix 服务器
            let server = HttpServer::new(|| {
                App::new()
                    .wrap(
                        actix_web::middleware::DefaultHeaders::new()
                            .add(("Access-Control-Allow-Origin", "*"))
                            .add(("Access-Control-Allow-Methods", "GET,POST,PUT,DELETE")),
                    )
                    .service(root)
                    .service(user_info)
                // 可以在此添加更多路由
            })
            .bind(&addr)
            .unwrap()
            .shutdown_timeout(5)
            .run(); // 设置优雅关闭超时

            let handle = server.handle(); // 在闭包内获取 handle
            let srv = server;

            // 等待关闭信号或服务器自然结束
            tokio::select! {
                _ = shutdown_receiver => {
                    log::info!("收到关闭信号 | Received shutdown signal");
                    handle.stop(true).await; // 使用句柄关闭服务器
                }
                _ = srv => {
                    log::info!("服务器自然终止 | Server terminated normally");
                }
            }
        }));

        self.shutdown_sender = Some(shutdown_sender);
        Ok(())
    }

    // 关闭服务器
    pub async fn stop(&mut self) {
        if let Some(sender) = self.shutdown_sender.take() {
            let _ = sender.send(());
        }
        if let Some(handle) = self.server_handle.take() {
            let _ = handle.await;
        }
        log::info!("服务器已关闭 | Server stopped");
    }
}

#[tauri::command]
pub async fn run_server<R: tauri::Runtime>(
    wenfox: tauri::AppHandle<R>, _window: tauri::Window<R>,
) -> Result<(), String> {
    let server = wenfox.state::<std::sync::Arc<tokio::sync::Mutex<crate::server::WenfoxServer>>>();
    let mut server = server.lock().await;
    server
        .run()
        .map_err(|e| format!("无法启动服务器 | Failed to start server: {}", e))?;
    log::info!("服务器已启动 | Server started");
    Ok(())
}

#[tauri::command]
pub async fn stop_server<R: tauri::Runtime>(
    wenfox: tauri::AppHandle<R>, _window: tauri::Window<R>,
) -> Result<(), String> {
    let server = wenfox.state::<std::sync::Arc<tokio::sync::Mutex<crate::server::WenfoxServer>>>();
    let mut server = server.lock().await;

    server.stop().await;
    log::info!("服务器已关闭 | Server stopped");
    Ok(())
}

// 重启服务器
// restart server
// 添加可选的重试次数参数
// add optional retry count parameter
// example:
// // // 使用默认3次重试
// await invoke('restart_server');
//
// // 自定义重试5次
// await invoke('restart_server', { maxRetries: 5 });
//
#[tauri::command]
pub async fn restart_server<R: tauri::Runtime>(
    wenfox: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    max_retries: Option<u8>, // 添加可选的重试次数参数
) -> Result<(), String> {
    let retries = max_retries.unwrap_or(3);
    let server = wenfox.state::<std::sync::Arc<tokio::sync::Mutex<crate::server::WenfoxServer>>>();
    let mut server = server.lock().await;

    server.stop().await;
    log::info!(
        "正在尝试重启服务器（剩余重试次数 {}）| Restarting server (retries left: {})",
        retries,
        retries
    );

    let mut attempts = 0;
    while attempts < retries {
        match server.run() {
            Ok(_) => {
                log::info!("服务器重启成功 | Server restarted successfully");
                return Ok(());
            }
            Err(e) => {
                attempts += 1;
                log::warn!(
                    "重启失败第 {} 次（共 {} 次）| Restart failed attempt {}/{}: {}",
                    attempts,
                    retries,
                    attempts,
                    retries,
                    e
                );

                // 等待1秒后重试
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }
    }

    Err(format!(
        "经过 {} 次尝试后仍无法重启服务器 | Failed to restart after {} attempts",
        retries, retries
    ))
}
