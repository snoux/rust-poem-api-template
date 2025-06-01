//! API模块
//! 
//! 包含所有API接口的定义，每个子模块代表一个功能域

pub mod user;
pub mod system;

use poem_openapi::{OpenApiService, ContactObject, LicenseObject};

/// 创建OpenAPI服务
/// 
/// 聚合所有API模块，并配置OpenAPI文档
pub fn create_api_service() -> OpenApiService<(user::UserController, system::SystemController), ()> {
    OpenApiService::new(
        (
            user::UserController::default(), // 用户管理API控制器
            system::SystemController::default(), // 系统管理API控制器
        ),
        "Rust Poem API Template", // API标题
        env!("CARGO_PKG_VERSION"), // API版本（从Cargo.toml获取）
    )
    .server("/api") // API基础路径
    .description("一个基于Rust和Poem框架的模块化后端API模板") // API描述
    .contact(ContactObject::new()
        .name("API Template Creator")
        .url("https://github.com/your-username/rust-poem-api-template")
        .email("example@example.com")
    ) // 联系人信息
    .license(LicenseObject::new("MIT")
        .url("https://opensource.org/licenses/MIT")
    ) // 许可证
    .external_document("https://github.com/your-username/rust-poem-api-template") // 外部文档
}
