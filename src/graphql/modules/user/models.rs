use async_graphql::SimpleObject;
use crate::models::common::{UserBase, BaseUser};

/// 用户模型
#[derive(SimpleObject, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
}

// 实现UserBase特性，支持与REST API模型的转换
impl UserBase for User {
    fn id(&self) -> Option<u64> {
        Some(self.id as u64)
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

// 允许从BaseUser转换为GraphQL User的实现
impl From<BaseUser> for User {
    fn from(base: BaseUser) -> Self {
        Self {
            id: base.id.unwrap_or(0) as i32,
            name: base.name,
        }
    }
}

// 允许从REST API User转换为GraphQL User的实现
impl From<crate::models::user::User> for User {
    fn from(rest_user: crate::models::user::User) -> Self {
        Self {
            id: rest_user.id.unwrap_or(0) as i32,
            name: rest_user.username,
        }
    }
}
