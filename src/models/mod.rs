//! 数据模型定义
//! 
//! 本模块包含应用程序中使用的所有数据模型定义。

pub mod user;
pub mod common; // 新增通用模型模块

// 重新导出常用模型，方便其他模块引用
pub use user::User;
pub use common::{BaseUser, UserBase, ErrorResponse, PaginationParams};
