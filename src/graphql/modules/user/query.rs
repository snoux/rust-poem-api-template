// src/graphql/modules/user/query.rs

use async_graphql::{Context, Object, Result};
use super::models::User;

/// 用户查询操作
#[derive(Default)] // 添加 Default 派生
pub struct UserQuery;

#[Object]
impl UserQuery {
    /// 获取所有用户
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        // 实际项目中从数据库获取
        Ok(vec![
            User { id: 1, name: "Alice".to_string() },
            User { id: 2, name: "Bob".to_string() },
        ])
    }

    /// 根据ID获取用户
    async fn user(&self, _ctx: &Context<'_>, id: i32) -> Result<Option<User>> {
        // 模拟数据库查询
        Ok(Some(User { id, name: "Test User".to_string() }))
    }
}