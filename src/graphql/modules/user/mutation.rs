// src/graphql/modules/user/mutation.rs

use async_graphql::{Context, Object, Result};
use super::models::User;
use crate::graphql::error::{graphql_error, GraphQLErrorType};

/// 用户变更操作
#[derive(Default)] // 添加 Default 派生
pub struct UserMutation;

#[Object]
impl UserMutation {
    /// 创建新用户
    /// 
    /// 根据提供的用户名创建新用户
    /// 返回创建成功的用户信息
    async fn create_user(&self, _ctx: &Context<'_>, name: String) -> Result<User> {
        // 验证用户名
        if name.len() < 3 {
            return Err(graphql_error(
                GraphQLErrorType::Validation,
                "用户名至少需要3个字符"
            ));
        }
        
        // 实际项目中保存到数据库
        Ok(User { id: 100, name })
    }
    
    /// 更新用户信息
    /// 
    /// 根据用户ID更新用户名
    /// 返回更新后的用户信息
    async fn update_user(&self, _ctx: &Context<'_>, id: i32, name: String) -> Result<User> {
        // 验证用户ID
        if id <= 0 {
            return Err(graphql_error(
                GraphQLErrorType::Validation,
                "用户ID必须为正整数"
            ));
        }
        
        // 验证用户名
        if name.len() < 3 {
            return Err(graphql_error(
                GraphQLErrorType::Validation,
                "用户名至少需要3个字符"
            ));
        }
        
        // 模拟更新操作
        // 实际应用中应先检查用户是否存在
        Ok(User { id, name })
    }
    
    /// 删除用户
    /// 
    /// 根据用户ID删除用户
    /// 返回操作是否成功
    async fn delete_user(&self, _ctx: &Context<'_>, id: i32) -> Result<bool> {
        // 验证用户ID
        if id <= 0 {
            return Err(graphql_error(
                GraphQLErrorType::Validation,
                "用户ID必须为正整数"
            ));
        }
        
        // 模拟删除操作
        // 实际应用中应先检查用户是否存在
        Ok(true)
    }
}
