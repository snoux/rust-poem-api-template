use async_graphql::MergedObject;
use crate::graphql::modules;

/// 组合所有模块的查询
#[derive(MergedObject, Default)]
pub struct Query(
    modules::user::query::UserQuery,
    // 添加新模块的查询类型
);