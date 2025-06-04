// src/graphql/modules/user/query.rs

use async_graphql::{Context, Object, Result};
use super::models::User;
use crate::graphql::error::{graphql_error, GraphQLErrorType};

/// 用户查询操作
#[derive(Default)] // 添加 Default 派生
pub struct UserQuery;

#[Object]
impl UserQuery {
    /// 获取所有用户
    /// 
    /// 返回系统中所有用户的列表
    async fn users(&self, _ctx: &Context<'_>) -> Result<Vec<User>> {
        // 实际项目中从数据库获取
        // 这里使用模拟数据进行演示
        Ok(vec![
            User { id: 1, name: "Alice".to_string() },
            User { id: 2, name: "Bob".to_string() },
        ])
    }

    /// 根据ID获取用户
    /// 
    /// 根据提供的用户ID查询并返回用户信息
    /// 如果用户不存在，返回None
    async fn user(&self, _ctx: &Context<'_>, id: i32) -> Result<Option<User>> {
        // 模拟数据库查询
        // 在实际应用中，应该检查用户是否存在
        if id <= 0 {
            return Err(graphql_error(
                GraphQLErrorType::Validation,
                "用户ID必须为正整数"
            ));
        }
        
        // 模拟查询结果
        Ok(Some(User { id, name: "Test User".to_string() }))
    }
    
    /// 根据用户名搜索用户
    /// 
    /// 根据提供的用户名模糊匹配用户
    async fn search_users(&self, _ctx: &Context<'_>, name_contains: String) -> Result<Vec<User>> {
        // 验证搜索参数
        if name_contains.len() < 2 {
            return Err(graphql_error(
                GraphQLErrorType::Validation,
                "搜索关键词至少需要2个字符"
            ));
        }
        
        // 模拟搜索结果
        Ok(vec![
            User { id: 1, name: format!("User with {}", name_contains) },
        ])
    }
}
