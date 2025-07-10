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


/// 从 GraphQL Error 中提取 ErrorResponse
pub fn to_error_response(error: &Error) -> ErrorResponse {
    let code = extract_string_extension(error, "code").unwrap_or("UNKNOWN_ERROR".into());
    let details = extract_string_extension(error, "details");

    ErrorResponse {
        code,
        message: error.message.clone(),
        details,
    }
}

/// 提取 extensions 字段中的字符串值
fn extract_string_extension(error: &Error, key: &str) -> Option<String> {
    error
        .extensions
        .as_ref()
        .and_then(|ext| ext.get(key))
        .and_then(|val| match val {
            Value::String(s) => Some(s.clone()),
            _ => None,
        })
}
