use anyhow::{Context, Result};
use argon2::Config;
use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header};
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;

use crate::domain::users::Users;

#[derive(Debug)]
pub struct CryptoService {
    pub hash_salt: Arc<String>,
    pub hash_secret: Arc<String>,
    pub jwt_secret: Arc<String>,
    pub access_expires: Arc<Duration>,
    pub refash_expires: Arc<Duration>,
    pub issuer: Arc<String>,
}
#[async_trait]
pub trait ExtCryptoService {
    /// 计算密码哈希
    async fn generate_password_hash(&self, password: &str) -> Result<String>;

    /// 验证密码哈希
    async fn verify_password(&self, password: &str, password_hash: &str) -> Result<bool>;

    /// 生成jwt
    async fn generate_jwt(&self, user: &Users) -> Result<(String, String)>;

    // 验证jwt
    // async fn verify_jwt(&self, token: &str) -> Result<bool>;
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: i64,    // 必填（验证中的defaultate_exp默认为true）。到期时间（以UTC时间戳记）
    iat: i64,    // 可选 签发时间（以UTC时间戳记）
    iss: String, // 可选 签发人
    nbf: i64,    // 可选 生效时间（以UTC时间戳记）
    sub: String, // 可选 用户
}

#[async_trait]
impl ExtCryptoService for CryptoService {
    /// 计算密码哈希
    async fn generate_password_hash(&self, pwd: &str) -> Result<String> {
        let config = Config {
            secret: self.hash_secret.as_bytes(),
            ..Config::default()
        };
        let salt = self.hash_salt.as_bytes();
        argon2::hash_encoded(pwd.as_bytes(), salt, &config).context("计算密码哈希异常!")
    }
    /// 验证密码哈希
    async fn verify_password(&self, pwd: &str, encoded: &str) -> Result<bool> {
        let secret = self.hash_secret.as_bytes();
        let pwd = pwd.as_bytes();
        argon2::verify_encoded_ext(encoded, pwd, secret, &[]).context("验证密码哈希异常!")
    }

    /// 生成jwt (access_token, refash_token)
    async fn generate_jwt(&self, user: &Users) -> Result<(String, String)> {
        let secret = &EncodingKey::from_secret(self.jwt_secret.as_bytes());
        let iss = self.issuer.to_string();
        let expires = *self.access_expires;

        let sub = user.id.to_string();
        let header = Header::default();
        let now = Utc::now();
        let exp = Utc::now() + expires;
        let claims = Claims {
            exp: exp.timestamp(),
            iat: now.timestamp(),
            nbf: now.timestamp(),
            iss,
            sub,
        };
        let access_token = jsonwebtoken::encode(&header, &claims, secret)?;

        let expires = *self.refash_expires;
        let exp = Utc::now() + expires;
        let claims = Claims {
            exp: exp.timestamp(),
            ..claims
        };
        let refash_token = jsonwebtoken::encode(&header, &claims, secret)?;
        Ok((access_token, refash_token))
    }
}
