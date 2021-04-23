use actix_web::{HttpResponse, Result, web};
use async_graphql::{EmptySubscription, Schema};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::{Request, Response};
use sqlx::PgPool;

use queries::Query;

use crate::gql::mutations::Mutation;

pub mod queries;
pub mod mutations;

/// 为了代码简洁, 定义 `ActixSchema`
type ServiceSchema = Schema<
    Query,
    Mutation,
    EmptySubscription
>;

pub async fn build_schema(pool: PgPool) -> ServiceSchema {
    // query 和 Mutation的根对象，并使用 EmptySubscription。
    // 在架构对象中添加全局sql数据源。
    Schema::build(Query, Mutation, EmptySubscription)
        .data(pool)
        .finish()
}

pub async fn graphql(schema: web::Data<ServiceSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(
        playground_source(
            // TODO: 2021-04-22 01:37:25 配置文件注入
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql")
        )
    ))
}
