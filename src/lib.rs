//! 项目入口模块
//! 
//! 这个文件是整个应用程序的入口点，负责初始化日志、创建API服务、配置路由和启动HTTP服务器。

pub mod api;
pub mod models;
pub mod services;
pub mod utils;
pub mod config;
pub mod middlewares;

// 重新导出一些常用模块，方便其他模块引用
pub use api::create_api_service;
pub use utils::response::{ApiResponse, success_json, error_json};
