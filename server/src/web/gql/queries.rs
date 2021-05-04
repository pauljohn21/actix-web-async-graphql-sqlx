use crate::common::error::errors::AppError;
use crate::domain::users::{TestValidator, Users};
use crate::service::users::{ExtUsersService, UsersService};
use crate::State;
use async_graphql::*;
use sqlx::PgPool;
use std::sync::Arc;

/// 定义查询根节点
#[derive(MergedObject, Default)]
pub struct QueryRoot(PingQuery, UsersQuery);

/// ping Query
#[derive(Default)]
pub struct PingQuery;

/// 用户查询 queries
#[derive(Default)]
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
        let pool = ctx.data::<Arc<State>>()?.pool.clone();
        Ok(UsersService::find_by_username(&pool, &username)
            .await
            .map_err(AppError::InternalError.log_extend())?)
    }

    /// 根据用户名查询用户2
    async fn find_by_username2(&self, ctx: &Context<'_>, username: String) -> FieldResult<Users> {
        let pool = ctx.data::<Arc<State>>()?.pool.clone();
        Ok(UsersService::find_by_username2(&pool, &username)
            .await
            .map_err(AppError::InternalError.log_extend())?)
    }

    /// 检查用户名是否存在
    async fn exists_by_username(&self, ctx: &Context<'_>, username: String) -> FieldResult<bool> {
        let pool = ctx.data::<Arc<State>>()?.pool.clone();
        Ok(UsersService::exists_by_username(&pool, &username)
            .await
            .map_err(AppError::InternalError.log_extend())?)
    }

    /// 测试graphql自带的字段验证器
    async fn test_validator(&self, tv: TestValidator) -> FieldResult<String> {
        Ok(tv.email)
    }
}
