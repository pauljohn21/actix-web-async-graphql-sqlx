use anyhow::Result;
use async_graphql::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
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
}