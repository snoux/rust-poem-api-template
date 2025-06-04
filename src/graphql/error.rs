//! GraphQL错误处理模块
//! 
//! 提供统一的GraphQL错误处理机制，确保错误响应格式一致

use async_graphql::{Error, ErrorExtensions};
use crate::models::common::ErrorResponse;

/// GraphQL错误类型
#[derive(Debug, Clone)]
pub enum GraphQLErrorType {
    /// 未找到资源
    NotFound,
    /// 验证错误
    Validation,
    /// 未授权
    Unauthorized,
    /// 禁止访问
    Forbidden,
    /// 内部服务器错误
    Internal,
}

impl GraphQLErrorType {
    /// 获取错误代码
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotFound => "NOT_FOUND",
            Self::Validation => "VALIDATION_ERROR",
            Self::Unauthorized => "UNAUTHORIZED",
            Self::Forbidden => "FORBIDDEN",
            Self::Internal => "INTERNAL_SERVER_ERROR",
        }
    }
}

/// 创建GraphQL错误
pub fn graphql_error(error_type: GraphQLErrorType, message: impl Into<String>) -> Error {
    let message = message.into();
    let code = error_type.code().to_string();
    
    Error::new(message.clone())
        .extend_with(|_, e| {
            e.set("code", code);
            e.set("type", format!("{:?}", error_type));
        })
}

/// 将GraphQL错误转换为通用错误响应
pub fn to_error_response(error: &Error) -> ErrorResponse {
    let extensions = error.extensions();
    let code = extensions
        .get("code")
        .and_then(|c| c.as_str())
        .unwrap_or("UNKNOWN_ERROR")
        .to_string();
    
    ErrorResponse {
        code,
        message: error.message.clone(),
        details: None,
    }
}
