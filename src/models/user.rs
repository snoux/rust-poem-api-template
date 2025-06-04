use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use super::common::{UserBase, BaseUser};

/// 用户模型
/// 
/// 用于表示系统中的用户实体
#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct User {
    /// 用户唯一标识符
    #[oai(read_only)]
    pub id: Option<u64>,
    
    /// 用户名，用于登录
    #[oai(validator(min_length = 3, max_length = 50))]
    pub username: String,
    
    /// 用户邮箱
    #[oai(validator(pattern = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"))]
    pub email: String,
    
    /// 用户创建时间（ISO 8601格式）
    #[oai(read_only)]
    pub created_at: Option<String>,
    
    /// 用户最后更新时间（ISO 8601格式）
    #[oai(read_only)]
    pub updated_at: Option<String>,
}

// 实现UserBase特性，支持与GraphQL模型的转换
impl UserBase for User {
    fn id(&self) -> Option<u64> {
        self.id
    }
    
    fn name(&self) -> &str {
        &self.username
    }
}

// 允许从BaseUser转换为User的实现
impl From<BaseUser> for User {
    fn from(base: BaseUser) -> Self {
        Self {
            id: base.id,
            username: base.name,
            email: String::new(), // 需要外部设置
            created_at: None,
            updated_at: None,
        }
    }
}

/// 用户创建请求
/// 
/// 用于创建新用户时的请求体
#[derive(Debug, Serialize, Deserialize, Object)]
pub struct CreateUserRequest {
    /// 用户名，用于登录
    #[oai(validator(min_length = 3, max_length = 50))]
    pub username: String,
    
    /// 用户邮箱
    #[oai(validator(pattern = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"))]
    pub email: String,
    
    /// 用户密码
    #[oai(validator(min_length = 6, max_length = 100))]
    pub password: String,
}

/// 用户更新请求
/// 
/// 用于更新现有用户信息的请求体
#[derive(Debug, Serialize, Deserialize, Object)]
pub struct UpdateUserRequest {
    /// 用户邮箱（可选）
    #[oai(validator(pattern = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"))]
    pub email: Option<String>,
    
    /// 用户密码（可选）
    #[oai(validator(min_length = 6, max_length = 100))]
    pub password: Option<String>,
}

/// 用户查询参数
/// 
/// 用于查询用户列表时的过滤条件
#[derive(Debug, Serialize, Deserialize, Object)]
pub struct UserQuery {
    /// 用户名模糊匹配
    pub username: Option<String>,
    
    /// 邮箱模糊匹配
    pub email: Option<String>,
    
    /// 分页：页码，从1开始
    #[oai(default = "default_page")]
    pub page: u32,
    
    /// 分页：每页记录数
    #[oai(default = "default_page_size")]
    pub page_size: u32,
}

/// 默认页码
fn default_page() -> u32 {
    1
}

/// 默认每页记录数
fn default_page_size() -> u32 {
    10
}

/// 用户列表响应
/// 
/// 用于返回用户列表查询结果
#[derive(Debug, Serialize, Deserialize, Object)]
pub struct UserListResponse {
    /// 用户列表
    pub users: Vec<User>,
    
    /// 总记录数
    pub total: u64,
    
    /// 当前页码
    pub page: u32,
    
    /// 每页记录数
    pub page_size: u32,
}
