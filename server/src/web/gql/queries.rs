use crate::common::error::errors::AppError;
use crate::domain::users::Users;
use crate::service::users::{ExtUsersService, UsersService};
use async_graphql::*;
use sqlx::PgPool;
use uuid::Uuid;

/// 定义查询根节点
#[derive(MergedObject, Default)]
pub struct QueryRoot(PingQuery, UsersQuery);

/// ping Query
#[derive(Default, Debug)]
pub struct PingQuery;

/// 用户查询 queries
#[derive(Default, Debug)]
pub struct UsersQuery;

#[Object]
impl PingQuery {
    async fn ping(&self) -> FieldResult<String> {
        Ok("pong".to_string())
    }
}

#[Object]
impl UsersQuery {
    /// 根据用户名查询用户
    async fn find_by_username(
        &self,
        ctx: &Context<'_>,
        username: String,
    ) -> FieldResult<Option<Users>> {
        let pool = ctx.data::<PgPool>()?;
        Ok(UsersService::find_by_username(pool, &username)
            .await
            .map_err(AppError::InternalError.log_extend())?)
    }

    /// 根据用户名查询用户2
    async fn find_by_username2(&self, ctx: &Context<'_>, username: String) -> FieldResult<Users> {
        let pool = ctx.data::<PgPool>()?;
        Ok(UsersService::find_by_username2(pool, &username)
            .await
            .map_err(AppError::InternalError.log_extend())?)
    }

    /// 检查用户名是否存在
    #[tracing::instrument(skip(ctx))]
    async fn exists_by_username(&self, ctx: &Context<'_>, username: String) -> FieldResult<bool> {
        let pool = ctx.data::<PgPool>()?;
        Ok(UsersService::exists_by_username(pool, &username)
            .await
            .map_err(AppError::InternalError.log_extend())?)
    }
}
