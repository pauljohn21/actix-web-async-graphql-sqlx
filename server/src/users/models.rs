use async_graphql::SimpleObject;
use sqlx::FromRow;
use sqlx::types::chrono::{DateTime, Utc};

/// 用户模型
#[derive(SimpleObject, FromRow)]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub email: String,
    password_hash: String,
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(SimpleObject, FromRow)]
pub struct UsersDTO {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


