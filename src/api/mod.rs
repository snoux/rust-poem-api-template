//! API模块
//! 
//! 包含所有API接口的定义，每个子模块代表一个功能域

pub mod user;

use poem_openapi::{OpenApiService, ContactObject, LicenseObject};

/// 创建OpenAPI服务
/// 
/// 聚合所有API模块，并配置OpenAPI文档
pub fn create_api_service() -> OpenApiService<(user::UserController), ()> {
    OpenApiService::new(
        (
            user::UserController::default() // 用户管理API控制器
        ),
        "{{ documentTitle }}", // API标题
        env!("CARGO_PKG_VERSION"), // API版本（从Cargo.toml获取）
    )
    .server("/api") // API基础路径
    .description("{{ description }}") // API描述
    .contact(ContactObject::new()
        .name("{{ authors }}")
        .url("{{ github }}")
        // .email("livefei@live.com")
    ) // 联系人信息
    .license(LicenseObject::new("MIT")
        .url("https://opensource.org/licenses/MIT")
    ) // 许可证
    // .external_document("https://github.com/snow-xf/rust-poem-api-template") // 外部文档
}
