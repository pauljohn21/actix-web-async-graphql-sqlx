use async_graphql::*;
use validator::Validate;

use crate::domain::users::{TestValidator, Users, UsersToken};
use crate::service::users::{ExtUsersService, UsersService};
use crate::web::gql::GraphqlResult;
use crate::State;
use crate::{common::error::errors::AppError, domain::users::LoginVM, EMAIL_REGEX};

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
    /// 用户登录
    async fn user_sign_in(&self, ctx: &Context<'_>, vm: LoginVM) -> GraphqlResult<UsersToken> {
        // 参数校验
        vm.validate()
            .map_err(AppError::RequestParameterError.validation_extend())?;
        // 把登录名称处理为小写
        let login = vm.login.to_lowercase();
        // 先判断用户登录方式
        let is_email_login = EMAIL_REGEX.is_match(&login);

        // 获取数据库连接池
        let pool = State::get_pool(ctx)?;

        let users = if is_email_login {
            UsersService::find_by_email(&pool, &login).await?
        } else {
            UsersService::find_by_username(&pool, &login).await?
        };

        // 判断用户是否存在
        let users = match users {
            Some(users) => users,
            None => return Err(AppError::UsernameOrPasswordError.extend()),
        };

        // 验证密码是否正确
        let crypto = State::get_crypto_server(ctx)?;

        let verify = crypto
            .verify_password(&vm.password, &users.password_hash)
            .await;

        // 处理验证结果
        match verify {
            // 验证出现异常
            Err(_) => return Err(AppError::InternalError.extend()),
            // 验证不通过
            Ok(verify) if !verify => return Err(AppError::UsernameOrPasswordError.extend()),
            // 验证通过
            _ => log::info!("用户: [{}] 登录成功", &users.username),
        };

        // todo!("生成jsonwebtoken并返回");
        // todo 代码抽到 service 层去 这一次进做参数校验

        let (access_token, refash_token, expires) = crypto.generate_jwt(&users.id).await?;

        let users_token = UsersToken {
            access_token,
            refash_token,
            expires: expires.num_seconds(),
        };

        Ok(users_token)
    }

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
