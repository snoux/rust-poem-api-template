// src/graphql/modules/user/mutation.rs

use async_graphql::{Context, Object, Result};
use super::models::User;

/// 用户变更操作
#[derive(Default)] // 添加 Default 派生
pub struct UserMutation;

#[Object]
impl UserMutation {
    /// 创建新用户
    async fn create_user(&self, _ctx: &Context<'_>, name: String) -> Result<User> {
        // 实际项目中保存到数据库
        Ok(User { id: 100, name })
    }
}