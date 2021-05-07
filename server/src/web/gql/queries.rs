use async_graphql::*;

use crate::common::error::errors::AppError;
use crate::domain::users::{TestValidator, Users};
use crate::service::users::{ExtUsersService, UsersService};
use crate::web::gql::GraphqlResult;
use crate::State;

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
    async fn ping(&self) -> GraphqlResult<String> {
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
    ) -> GraphqlResult<Option<Users>> {
        let pool = State::get_pool(ctx)?;
        Ok(UsersService::find_by_username(&pool, &username)
            .await
            .map_err(AppError::InternalError.log_extend())?)
    }

    /// 根据用户名查询用户2
    async fn find_by_username2(&self, ctx: &Context<'_>, username: String) -> GraphqlResult<Users> {
        let pool = State::get_pool(ctx)?;
        Ok(UsersService::find_by_username2(&pool, &username)
            .await
            .map_err(AppError::InternalError.log_extend())?)
    }

    /// 检查用户名是否存在
    async fn exists_by_username(&self, ctx: &Context<'_>, username: String) -> GraphqlResult<bool> {
        let pool = State::get_pool(ctx)?;
        Ok(UsersService::exists_by_username(&pool, &username)
            .await
            .map_err(AppError::InternalError.log_extend())?)
    }

    /// 测试graphql自带的字段验证器
    async fn test_validator(&self, tv: TestValidator) -> GraphqlResult<String> {
        Ok(tv.email)
    }
}
