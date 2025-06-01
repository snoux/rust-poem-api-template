use crate::models::user::{CreateUserRequest, UpdateUserRequest, User, UserListResponse};
use crate::utils::response::{success_json, error_json, ApiResponse, EmptyResponse, empty};
use poem::Result;
use poem_openapi::{
    param::{Path, Query},
    payload::Json,
    OpenApi,
};
use  crate::config::tags::ApiTags;

use super::dto::*;

/// 用户管理API控制器
/// 
/// 提供用户相关的所有RESTful接口
#[derive(Default)]
pub struct UserController;

#[OpenApi]
impl UserController {
    /// 创建新用户
    /// 
    /// 根据提供的用户信息创建一个新用户
    #[oai(path = "/users", method = "post", operation_id = "createUser", tag = ApiTags::User)]
    async fn create_user(&self, req: Json<CreateUserRequest>) -> Result<Json<ApiResponse<User>>> {
        // 这里是模拟实现，实际项目中应该调用service层处理业务逻辑
        let user = User {
            id: Some(1),
            username: req.0.username,
            email: req.0.email,
            created_at: Some(chrono::Utc::now().to_rfc3339()),
            updated_at: Some(chrono::Utc::now().to_rfc3339()),
        };
        
        // 返回统一格式的成功响应
        success_json(user)
    }
    
    /// 获取用户详情
    /// 
    /// 根据用户ID获取用户详细信息
    #[oai(path = "/users/:id", method = "get", operation_id = "getUserById", tag = ApiTags::User)]
    async fn get_user(&self, id: Path<u64>) -> Result<Json<ApiResponse<User>>> {
        // 这里是模拟实现，实际项目中应该调用service层处理业务逻辑
        // 模拟用户查找
        if id.0 > 100 {
            // 用户不存在，返回错误
            return error_json(404, format!("User with id {} not found", id.0));
        }
        
        let user = User {
            id: Some(id.0),
            username: format!("user_{}", id.0),
            email: format!("user_{}@example.com", id.0),
            created_at: Some("2025-01-01T00:00:00Z".to_string()),
            updated_at: Some("2025-01-01T00:00:00Z".to_string()),
        };
        
        // 返回统一格式的成功响应
        success_json(user)
    }
    
    /// 更新用户信息
    /// 
    /// 根据用户ID更新用户信息
    #[oai(path = "/users/:id", method = "put", operation_id = "updateUser", tag = ApiTags::User)]
    async fn update_user(&self, id: Path<u64>, req: Json<UpdateUserRequest>) -> Result<Json<ApiResponse<User>>> {
        // 这里是模拟实现，实际项目中应该调用service层处理业务逻辑
        // 模拟用户查找
        if id.0 > 100 {
            // 用户不存在，返回错误
            return error_json(404, format!("User with id {} not found", id.0));
        }
        
        // 模拟更新用户
        let email = req.0.email.unwrap_or_else(|| format!("user_{}@example.com", id.0));
        
        let user = User {
            id: Some(id.0),
            username: format!("user_{}", id.0),
            email,
            created_at: Some("2025-01-01T00:00:00Z".to_string()),
            updated_at: Some(chrono::Utc::now().to_rfc3339()),
        };
        
        // 返回统一格式的成功响应
        success_json(user)
    }
    
    /// 删除用户
    /// 
    /// 根据用户ID删除用户
    #[oai(path = "/users/:id", method = "delete", operation_id = "deleteUser", tag = ApiTags::User)]
    async fn delete_user(&self, id: Path<u64>) -> Result<Json<ApiResponse<EmptyResponse>>> {
        // 这里是模拟实现，实际项目中应该调用service层处理业务逻辑
        // 模拟用户查找
        if id.0 > 100 {
            // 用户不存在，返回错误
            return error_json(404, format!("User with id {} not found", id.0));
        }
        
        // 模拟删除用户
        // 返回统一格式的成功响应（无数据）
        success_json(empty())
    }
    
    /// 获取用户列表
    /// 
    /// 根据查询条件获取用户列表
    #[oai(path = "/users", method = "get", operation_id = "listUsers", tag = ApiTags::User)]
    async fn list_users(
        &self,
        /// 用户名模糊匹配
        #[oai(name = "username")] username: Query<Option<String>>,
        /// 邮箱模糊匹配
        #[oai(name = "email")] email: Query<Option<String>>,
        /// 分页：页码，从1开始
        #[oai(name = "page")] page: Query<Option<u32>>,
        /// 分页：每页记录数
        #[oai(name = "page_size")] page_size: Query<Option<u32>>,
    ) -> Result<Json<ApiResponse<UserListResponse>>> {
        // 这里是模拟实现，实际项目中应该调用service层处理业务逻辑
        let page = page.0.unwrap_or(1);
        let page_size = page_size.0.unwrap_or(10);
        
        let mut users = Vec::new();
        
        // 模拟分页
        let start = ((page - 1) * page_size) as usize;
        let end = start + page_size as usize;
        
        // 模拟用户列表
        for i in start..end {
            if i >= 100 {
                break;
            }
            
            // 模拟用户名和邮箱过滤
            let user_name = format!("user_{}", i + 1);
            let user_email = format!("user_{}@example.com", i + 1);
            
            // 如果提供了用户名过滤条件，但不匹配，则跳过
            if let Some(ref name_filter) = username.0 {
                if !user_name.contains(name_filter) {
                    continue;
                }
            }
            
            // 如果提供了邮箱过滤条件，但不匹配，则跳过
            if let Some(ref email_filter) = email.0 {
                if !user_email.contains(email_filter) {
                    continue;
                }
            }
            
            users.push(User {
                id: Some(i as u64 + 1),
                username: user_name,
                email: user_email,
                created_at: Some("2025-01-01T00:00:00Z".to_string()),
                updated_at: Some("2025-01-01T00:00:00Z".to_string()),
            });
        }
        
        let response = UserListResponse {
            users,
            total: 100,
            page,
            page_size,
        };
        
        // 返回统一格式的成功响应
        success_json(response)
    }
}
