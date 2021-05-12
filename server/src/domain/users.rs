use crate::USERNAME_REGEX;
use async_graphql::validators::Email;
use async_graphql::*;
use chrono::{DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

/// 用户模型
#[derive(SimpleObject, FromRow, Deserialize, Serialize)]
#[graphql(complex)]
pub struct Users {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[graphql(skip)]
    #[serde(skip_serializing)]
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
#[derive(Serialize, Deserialize, InputObject, Validate)]
pub struct NewUser {
    #[validate(regex(path = "USERNAME_REGEX", message = "用户名不符合要求"))]
    pub username: String,
    #[validate(email(message = "邮箱不符合"))]
    pub email: String,
    #[validate(length(min = 6, message = "密码不符合"))]
    pub password: String,
    #[validate(length(min = 3, message = "昵称不符合"))]
    pub nickname: String,
}

/// 用户注册
#[derive(Serialize, Deserialize, InputObject, Validate)]
pub struct LoginVM {
    #[validate(length(min = 1, message = "登录名称不符合要求"))]
    pub login: String,
    #[validate(length(min = 6, message = "密码不符合"))]
    pub password: String,
}

/// 测试Graphql的字段校验器
#[derive(Serialize, Deserialize, InputObject)]
pub struct TestValidator {
    #[graphql(validator(Email))]
    pub email: String,
}

/// 用户登录token结构体
#[derive(SimpleObject)]
pub struct UsersToken {
    pub access_token: String,
    pub refash_token: String,
    pub expires: i64,
}
