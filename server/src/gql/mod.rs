use async_graphql::{Schema, EmptyMutation, EmptySubscription};
use queries::QueryRoot;
use actix_web::{web, HttpResponse, Result};
use async_graphql_actix_web::{Request, Response};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use sqlx::PgPool;

pub mod queries;
pub mod mutations;

/// 为了代码简洁, 定义 `ActixSchema`
type ActixSchema = Schema<
    QueryRoot,
    EmptyMutation,
    EmptySubscription
>;

pub async fn build_schema(pool: PgPool) -> Schema<QueryRoot, EmptyMutation, EmptySubscription> {
    // query 和 Mutation的根对象，并使用 EmptySubscription。
    // 在架构对象中添加全局sql数据源。
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(pool)
        .finish()
}

pub async fn graphql(schema: web::Data<ActixSchema>, req: Request) -> Response {
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
