use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::users::{NewUser, Users};
use crate::repository::users::{ExtUsersRepository, UsersRepository};

pub struct UsersService;

#[async_trait]
pub trait ExtUsersService {
    /// 注册用户
    async fn user_register(pool: &PgPool, new_user: &NewUser, encoded: &str) -> Result<String>;

    /// 创建用户
    async fn create(
        pool: &PgPool,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<Users>;

    /// 根据用户名查询用户
    async fn find_by_username(pool: &PgPool, username: &str) -> Result<Option<Users>>;

    /// 根据用户名查询用户2
    async fn find_by_username2(pool: &PgPool, username: &str) -> Result<Users>;

    /// 检查用户是否存在
    async fn exists_by_username(pool: &PgPool, username: &str) -> Result<bool>;

    /// 检查邮箱是否存在
    async fn exists_by_email(pool: &PgPool, email: &str) -> Result<bool>;
}

#[async_trait]
impl ExtUsersService for UsersService {
    async fn user_register(_pool: &PgPool, _new_user: &NewUser, _encoded: &str) -> Result<String> {
        todo!()
    }

    async fn create(
        pool: &PgPool,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<Users> {
        UsersRepository::create(pool, username, email, password_hash).await
    }

    async fn find_by_username(pool: &PgPool, username: &str) -> Result<Option<Users>> {
        UsersRepository::find_by_username(pool, username).await
    }

    async fn find_by_username2(pool: &PgPool, username: &str) -> Result<Users> {
        UsersRepository::find_by_username2(pool, username).await
    }

    async fn exists_by_username(pool: &PgPool, username: &str) -> Result<bool> {
        UsersRepository::exists_by_username(pool, username).await
    }

    async fn exists_by_email(pool: &PgPool, email: &str) -> Result<bool> {
        UsersRepository::exists_by_email(pool, email).await
    }
}
