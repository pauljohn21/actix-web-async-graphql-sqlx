use chrono::{DateTime, Utc, Local};
use uuid::Uuid;
use async_graphql::*;
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;

/// 用户模型
#[derive(SimpleObject, FromRow, Deserialize, Serialize)]
#[graphql(complex)]
pub struct Users {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[graphql(skip)]
    pub password_hash: String,
    pub nickname: String,
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

/// 用户注册
#[derive(Serialize, Deserialize, InputObject)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub nickname: String,
}