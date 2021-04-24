use anyhow::Result;
use async_graphql::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use sqlx::PgPool;
use uuid::Uuid;

/// 用户模型
#[derive(SimpleObject, FromRow, Deserialize, Serialize)]
#[graphql(complex)]
pub struct Users {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[graphql(skip)]
    pub password_hash: String,
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub active: bool,
    pub email_verified: bool,
    #[graphql(skip)]
    pub created_at: DateTime<Utc>,
    #[graphql(skip)]
    pub updated_at: DateTime<Utc>,
}

#[ComplexObject]
impl Users {
    async fn created_at(&self) -> DateTime<Local> {
        self.created_at.with_timezone(&Local)
    }

    async fn updated_at(&self) -> DateTime<Local> {
        self.updated_at.with_timezone(&Local)
    }
}

/// 用户方法
impl Users {
    /// 创建用户
    pub async fn create(pool: &PgPool, username: &str, email: &str, password_hash: &str) -> Result<Users> {
        let row = sqlx::query_as!(
            Users,
            //language=sql
            "INSERT INTO users(username, email, password_hash) VALUES ($1, $2, $3) RETURNING *",
            username, email, password_hash
        ).fetch_one(pool).await?;

        Ok(row)
    }

    /// 查询用户
    pub async fn find_by_username(pool: &PgPool, username: &str) -> Result<Option<Users>> {
        let row = sqlx::query_as!(
            Users,
            //language=sql
            "SELECT * FROM users WHERE username = $1",
            username
        ).fetch_optional(pool).await?;

        Ok(row)
    }

    /// 检查用户是否存在
    pub async fn exists_by_username(pool: &PgPool, username: &str) -> Result<bool> {
        let row = sqlx::query!(
            //language=sql
            "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)",
            username
        ).fetch_one(pool).await?;
        let exists: Option<bool> = row.exists;
        Ok(exists.unwrap_or_default())
    }
}