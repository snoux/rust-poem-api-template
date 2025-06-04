//! 通用模型定义
//! 
//! 本模块包含在REST API和GraphQL之间共享的通用模型定义，
//! 提供了统一的数据结构和转换特性，减少代码重复。

use serde::{Serialize, Deserialize};

/// 用户基础信息特性
/// 
/// 定义用户模型的基本属性和行为，用于在不同API类型间共享
pub trait UserBase {
    /// 获取用户ID
    fn id(&self) -> Option<u64>;
    
    /// 获取用户名
    fn name(&self) -> &str;
    
    /// 创建一个基础用户模型
    fn to_base_user(&self) -> BaseUser {
        BaseUser {
            id: self.id(),
            name: self.name().to_string(),
        }
    }
}

/// 基础用户模型
/// 
/// 包含用户的核心属性，可用于REST API和GraphQL之间的转换
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseUser {
    /// 用户ID
    pub id: Option<u64>,
    
    /// 用户名
    pub name: String,
}

/// 错误响应
/// 
/// 统一的错误响应格式，可用于REST API和GraphQL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// 错误代码
    pub code: String,
    
    /// 错误消息
    pub message: String,
    
    /// 详细错误信息（可选）
    pub details: Option<String>,
}

/// 分页参数
/// 
/// 用于列表查询的分页参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    /// 页码，从1开始
    pub page: u32,
    
    /// 每页记录数
    pub page_size: u32,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 10,
        }
    }
}
