use std::sync::Arc;

use async_graphql::*;
use validator::Validate;

use crate::security::crypto::ExtCryptoService;
use crate::service::users::{ExtUsersService, UsersService};
use crate::{
    common::error::errors::AppError,
    domain::users::{NewUser, Users},
    State,
};

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
        let pool = ctx.data::<Arc<State>>()?.pool.clone();
        let users = UsersService::create(&pool, &username, &email, &password).await?;
        Ok(users)
    }

    /// 注册用户
    async fn user_register(&self, ctx: &Context<'_>, new_user: NewUser) -> FieldResult<String> {
        let pool = State::get_pool(ctx)?;
        let crypto = State::get_crypto_server(ctx)?;

        // 参数校验
        new_user
            .validate()
            .map_err(AppError::RequestParameterError.validation_extend())?;

        // 检查用户名重复
        let exists = UsersService::exists_by_username(&pool, &new_user.username).await?;
        if exists {
            return Err(AppError::UsernameAlreadyExists.extend());
        }

        // 检查邮箱重复
        let exists = UsersService::exists_by_email(&pool, &new_user.email).await?;
        if exists {
            return Err(AppError::UsernameAlreadyExists.extend());
        }

        // 密码哈希
        let encoded = crypto.hash_password(&new_user.password).await?;

        let uuid = UsersService::user_register(&pool, &new_user, &encoded).await?;
        Ok(uuid)
    }
}
