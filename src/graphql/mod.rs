// src/graphql/mod.rs

use async_graphql_poem::GraphQL;
use poem::{handler, Route, web::Html, get};
use async_graphql::{Schema, EmptySubscription, http::GraphiQLSource};

// 确保正确导入 Mutation
use crate::graphql::{query::Query, mutation::Mutation};

mod query;
mod mutation;
pub mod modules;

/// 创建GraphQL服务路由
pub fn create_graphql_route() -> Route {
    let schema = Schema::build(
        Query::default(),    // 默认查询对象
        Mutation::default(), // 默认变更对象
        EmptySubscription    // 空订阅
    )
    .finish();

    // 创建包含GraphQL Playground和API端点的路由
    Route::new()
        // 添加GraphQL Playground界面
        .at("/", get(graphql_playground))
        // 添加GraphQL API端点
        .at("/query", GraphQL::new(schema))
}

/// GraphQL Playground界面处理函数
#[handler]
async fn graphql_playground() -> Html<String> {
    // 构建GraphiQL界面，指定GraphQL端点
    Html(
        GraphiQLSource::build()
            .endpoint("/graphql/query") // 注意：这里的路径与上面的路由匹配
            .finish()
    )
}