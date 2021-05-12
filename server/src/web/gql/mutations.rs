use async_graphql::*;
use validator::*;

use crate::domain::users::Users;
use crate::service::users::{ExtUsersService, UsersService};
use crate::web::gql::GraphqlResult;
use crate::{common::error::errors::AppError, domain::users::NewUser, State};

/// 变更根节点
#[derive(MergedObject, Default)]
pub struct MutationRoot(UsersMutation);

/// 用户变更 Mutation
#[derive(Default)]
pub struct UsersMutation;

#[Object]
impl UsersMutation {
    /// 注册用户
    async fn user_register(
        &self,
        ctx: &Context<'_>,
        mut new_user: NewUser,
    ) -> GraphqlResult<Users> {
        // 参数校验
        new_user
            .validate()
            .map_err(AppError::RequestParameterError.validation_extend())?;

        let pool = State::get_pool(ctx)?;
        let crypto = State::get_crypto_server(ctx)?;

        // 处理为 小写
        new_user.username.make_ascii_lowercase();
        new_user.email.make_ascii_lowercase();

        // 检查用户名重复
        let exists = UsersService::exists_by_username(&pool, &new_user.username).await?;
        if exists {
            return Err(AppError::UsernameAlreadyExists.extend());
        }

        // 检查邮箱重复
        let exists = UsersService::exists_by_email(&pool, &new_user.email).await?;
        if exists {
            return Err(AppError::EmailAlreadyExists.extend());
        }

        // 密码哈希
        let password_hash = crypto.generate_password_hash(&new_user.password).await?;

        let user = UsersService::user_register(&pool, &new_user, &password_hash).await?;
        Ok(user)
    }
}
