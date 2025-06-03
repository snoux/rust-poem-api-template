use async_graphql::SimpleObject;

/// 用户模型
#[derive(SimpleObject, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
}