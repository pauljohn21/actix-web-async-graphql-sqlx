use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Result;
use crate::domain::users::Users;
use crate::repository::users::{UsersRepository, ExtUsersRepository};

pub struct UsersService;

#[async_trait]
pub trait ExtUsersService {
    /// 创建用户
    async fn create(pool: &PgPool, username: &str, email: &str, password_hash: &str) -> Result<Users>;

    /// 根据用户名查询用户
    async fn find_by_username(pool: &PgPool, username: &str) -> Result<Option<Users>>;

    /// 根据用户名查询用户2
    async fn find_by_username2(pool: &PgPool, username: &str) -> Result<Users>;

    /// 检查用户是否存在
    async fn exists_by_username(pool: &PgPool, username: &str) -> Result<bool>;
}

#[async_trait]
impl ExtUsersService for UsersService {
    async fn create(pool: &PgPool, username: &str, email: &str, password_hash: &str) -> Result<Users> {
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
}