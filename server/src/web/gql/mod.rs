use actix_web::{web, HttpResponse, Result};
use async_graphql::extensions::{ApolloTracing, Logger, Tracing};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};
use sqlx::PgPool;

use queries::QueryRoot;

use crate::config::configs::{Configs, GraphQlConfig};
use crate::gql::mutations::MutationRoot;
use std::sync::Arc;

pub mod mutations;
pub mod queries;

/// 为了代码简洁, 定义 `ServiceSchema`
pub type ServiceSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

/// 创建 Schema
pub async fn build_schema(pool: Arc<PgPool>, config: &GraphQlConfig) -> ServiceSchema {
    let builder = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(pool);
    if config.tracing.unwrap_or(false) {
        builder
            .extension(ApolloTracing)
            .extension(Tracing)
            .extension(Logger)
            .finish()
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
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            // TODO: 2021-04-22 01:37:25 配置文件注入
            GraphQLPlaygroundConfig::new(path).subscription_endpoint(path),
        )))
}
