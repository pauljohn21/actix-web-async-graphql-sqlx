use crate::service::users::{ExtUsersService, UsersService};
use actix_web::web::Data;
use actix_web::{get, HttpResponse, Responder};
use sqlx::PgPool;
use std::sync::Arc;

/// 查询用户名
#[tracing::instrument(skip(pool))]
pub async fn find_by_username(pool: Data<Arc<PgPool>>, username: String) -> impl Responder {
    tracing::debug!("rest 查询用户名");
    let result = UsersService::find_by_username(&pool, &username).await;
    tracing::debug!("rest 查询用户名  ->>> end");
    HttpResponse::Ok()
}
