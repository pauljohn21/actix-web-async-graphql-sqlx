use actix_web::{HttpResponse, Result, web};
use async_graphql::{EmptySubscription, Schema};
use async_graphql::extensions::ApolloTracing;
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::{Request, Response};
use sqlx::PgPool;

use queries::QueryRoot;

use crate::gql::mutations::MutationRoot;
use crate::config::configs::{GraphQlConfig, Configs};
use std::sync::Arc;

pub mod queries;
pub mod mutations;

/// 为了代码简洁, 定义 `ServiceSchema`
pub type ServiceSchema = Schema<
    QueryRoot,
    MutationRoot,
    EmptySubscription
>;

/// 创建 Schema
pub async fn build_schema(pool: PgPool, config: &GraphQlConfig) -> ServiceSchema {
    let builder = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription)
        .data(pool);
    if config.tracing.unwrap_or(false) {
        builder.extension(ApolloTracing).finish()
    } else {
        builder.finish()
    }
}

/// Schema 执行
pub async fn graphql(schema: web::Data<ServiceSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

/// 创建 GraphQLPlayground
pub async fn graphiql(config: web::Data<Arc<Configs>>) -> Result<HttpResponse> {
    let path = &config.graphql.path;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(
        playground_source(
            // TODO: 2021-04-22 01:37:25 配置文件注入
            GraphQLPlaygroundConfig::new(path).subscription_endpoint(path)
        )
    ))
}
