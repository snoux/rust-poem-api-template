// src/graphql/mutation.rs

use async_graphql::MergedObject;
use crate::graphql::modules;

/// 组合所有模块的变更操作
#[derive(MergedObject, Default)]
pub struct Mutation(
    modules::user::mutation::UserMutation,
    // 添加新模块的变更类型
);