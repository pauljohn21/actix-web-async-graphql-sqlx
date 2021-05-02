use crate::domain::users::Users;
use anyhow::*;
use async_trait::async_trait;
use sqlx::PgPool;
use tracing::Instrument;

pub struct UsersRepository;

#[async_trait]
pub trait ExtUsersRepository {
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

    /// 检查用户是否存在
    async fn exists_by_email(pool: &PgPool, email: &str) -> Result<bool>;
}

#[async_trait]
impl ExtUsersRepository for UsersRepository {
    /// 创建用户
    #[tracing::instrument(skip(pool))]
    async fn create(
        pool: &PgPool,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<Users> {
        let row = sqlx::query_as!(
            Users,
            //language=sql
            "INSERT INTO users(username, email, password_hash) VALUES ($1, $2, $3) RETURNING *",
            username,
            email,
            password_hash
        )
        .fetch_one(pool)
        .await
        .context("创建用户")?;

        Ok(row)
    }

    /// 根据用户名查询用户
    #[tracing::instrument(skip(pool))]
    async fn find_by_username(pool: &PgPool, username: &str) -> Result<Option<Users>> {
        let row = sqlx::query_as!(Users, "SELECT * FROM users WHERE username = $1", username)
            .fetch_optional(pool)
            .await
            .context("查询用户")?;

        Ok(row)
    }

    /// 根据用户名查询用户2
    #[tracing::instrument(skip(pool))]
    async fn find_by_username2(pool: &PgPool, username: &str) -> Result<Users> {
        let row = sqlx::query_as!(Users, "SELECT * FROM users WHERE username = $1", username)
            .fetch_one(pool)
            .await
            .context("根据用户名查询用户2")?;

        Ok(row)
    }

    /// 检查用户是否存在
    #[tracing::instrument(skip(pool))]
    async fn exists_by_username(pool: &PgPool, username: &str) -> Result<bool> {
        tracing::info!("检查用户是否存在");
        let row = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)",
            username,
        )
        .fetch_one(pool)
        .await
        .context("检查用户是否存在")?;
        let exists: Option<bool> = row.exists;
        Ok(exists.unwrap_or_default())
    }

    #[tracing::instrument(skip(pool))]
    async fn exists_by_email(pool: &PgPool, email: &str) -> Result<bool> {
        let row = sqlx::query!("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)", email,)
            .fetch_one(pool)
            .await
            .context("检查邮箱是否存在")?;
        let exists: Option<bool> = row.exists;
        Ok(exists.unwrap_or_default())
    }
}
