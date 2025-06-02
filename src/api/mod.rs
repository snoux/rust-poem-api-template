//! API模块
//! 
//! 包含所有API接口的定义，每个子模块代表一个功能域

pub mod user;

use poem_openapi::{OpenApiService, ContactObject, LicenseObject};

const AUTHOR: &str = "{{ author }}";
const GITHUB: &str = "{{ github }}";

/// 创建OpenAPI服务
/// 
/// 聚合所有API模块，并配置OpenAPI文档
pub fn create_api_service() -> OpenApiService<(user::UserController), ()> {
    let mut service = OpenApiService::new(
        (
            user::UserController::default() // 用户管理API控制器
        ),
        "{{ doc_title }}", // API文档标题
        env!("CARGO_PKG_VERSION"), // API版本（从Cargo.toml获取）
    )
    .server("/api") // API基础路径
    .description("{{ project_description }}") // API描述
    .license(
        LicenseObject::new("MIT")
            .url("https://opensource.org/licenses/MIT"),
    ); // 许可证

    if !AUTHOR.trim().is_empty() && !GITHUB.trim().is_empty() {
        service = service.contact(
            ContactObject::new()
                .name(AUTHOR)
                .url(GITHUB),
        );
    }


    service
}