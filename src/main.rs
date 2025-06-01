use poem::{
    listener::TcpListener,
    middleware::{Cors, Tracing},
    EndpointExt, Route, Server,
};
use rust_poem_api_template::api;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,poem=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 创建API服务
    let api_service = api::create_api_service();
    
    // 创建API文档服务（需要单独创建一个实例，避免所有权问题）
    let api_doc_service = api::create_api_service();
    
    // 创建路由
    let app = Route::new()
        // API路由
        .nest("/api", api_service)
        // OpenAPI规范JSON端点
        .nest("/api/docs/json", api_doc_service.spec_endpoint())
        // Swagger UI端点
        .nest("/api/docs", api_doc_service.swagger_ui())
        // 添加CORS中间件
        .with(Cors::new())
        // 添加日志中间件
        .with(Tracing);

    // 获取监听地址
    let addr = std::env::var("LISTEN_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:3000".to_string())
        .parse::<SocketAddr>()
        .expect("无效的监听地址");

    tracing::info!("服务启动在 http://{}", addr);
    tracing::info!("OpenAPI文档JSON可在 http://{}/api/docs 访问", addr);

    // 启动服务器
    Server::new(TcpListener::bind(addr))
        .run(app)
        .await
}
