use async_graphql::*;
use sqlx::PgPool;

use crate::service::users::{ExtUsersService, UsersService};
use crate::{
    common::error::errors::AppError,
    domain::users::{NewUser, Users},
};
use validator::Validate;

/// 变更根节点
#[derive(MergedObject, Default)]
pub struct MutationRoot(UsersMutation);

/// 用户变更 Mutation
#[derive(Default)]
pub struct UsersMutation;

#[Object]
impl UsersMutation {
    /// 创建用户
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        username: String,
        email: String,
        password: String,
    ) -> FieldResult<Users> {
        let pool = ctx.data::<PgPool>()?;
        let users = UsersService::create(pool, &username, &email, &password).await?;
        Ok(users)
    }

    /// 注册用户
    async fn user_register(&self, ctx: &Context<'_>, new_user: NewUser) -> FieldResult<String> {
        let pool = ctx.data::<PgPool>()?;

        // 参数校验
        new_user
            .validate()
            .map_err(AppError::RequestParameterError.validation_extend())?;

        // 检查用户名重复
        let exists = UsersService::exists_by_username(pool, &new_user.username).await?;
        if exists {
            return Err(AppError::UsernameAlreadyExists.extend());
        }

        // 检查邮箱重复
        let exists = UsersService::exists_by_email(pool, &new_user.email).await?;
        if exists {
            return Err(AppError::UsernameAlreadyExists.extend());
        }

        let token = UsersService::user_register(pool, &new_user).await?;
        Ok(token)
    }
}
