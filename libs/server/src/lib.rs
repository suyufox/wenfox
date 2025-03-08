use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};

// 定义根路径处理器
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Actix Web!")
}

// 带路径参数的处理器
#[get("/user/{name}")]
async fn user_info(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    HttpResponse::Ok().body(format!("Welcome, {}!", name))
}

pub struct WenfoxServer {
    host: String,
    port: u16,
    web: bool,
    proxy: bool,
}

impl WenfoxServer {
    /// 创建带默认的配置
    pub fn new() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 9803,
            web: true,
            proxy: false,
        }
    }

    /// 设置监听主机
    pub fn with_host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    /// 设置监听端口
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// 设置是否启用web服务
    pub fn with_web(mut self, web: bool) -> Self {
        self.web = web;
        self
    }

    /// 设置是否启用代理服务
    pub fn with_proxy(mut self, proxy: bool) -> Self {
        self.proxy = proxy;
        self
    }

    /// 启动服务器
    pub async fn run(self) -> std::io::Result<()> {
        let addr = format!("{}:{}", self.host, self.port);

        // 创建Tokio运行时
        HttpServer::new(|| {
            App::new()
                .wrap(
                    actix_web::middleware::DefaultHeaders::new()
                        .add(("Access-Control-Allow-Origin", "*"))
                        .add(("Access-Control-Allow-Methods", "GET,POST,PUT,DELETE")),
                )
                .service(hello)
                .service(user_info)
        })
        .bind(&addr)?
        .run()
        .await
    }
}
