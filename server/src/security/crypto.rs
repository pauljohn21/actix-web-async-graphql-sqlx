use anyhow::{Context, Result};
use argon2::Config;
use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode};
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use uuid::Uuid;

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
    async fn generate_jwt(&self, user_id: Uuid) -> Result<(String, String)>;

    // 验证jwt
    async fn verify_jwt(&self, token: &str) -> Result<TokenData<Claims>>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
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
    async fn generate_jwt(&self, user_id: Uuid) -> Result<(String, String)> {
        let secret = &EncodingKey::from_secret(self.jwt_secret.as_bytes());
        let iss = self.issuer.to_string();
        let expires = *self.access_expires;

        let sub = user_id.to_string();
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

    async fn verify_jwt(&self, token: &str) -> Result<TokenData<Claims>> {
        let secret = &DecodingKey::from_secret(self.jwt_secret.as_bytes());
        Ok(decode::<Claims>(token, secret, &Validation::default())?)
    }
}

#[actix_rt::test]
async fn test_generate_password_hash() {
    let crypto_service = CryptoService {
        hash_salt: Arc::new("test_generate_password_hash".to_string()),
        hash_secret: Arc::new("test_generate_password_hash".to_string()),
        jwt_secret: Arc::new("test_generate_password_hash".to_string()),
        access_expires: Arc::new(Duration::minutes(30)),
        refash_expires: Arc::new(Duration::days(7)),
        issuer: Arc::new("test".to_string()),
    };

    let pwd = "test_generate_password_hash";
    let encoded = crypto_service.generate_password_hash(pwd).await.unwrap();
    let x = crypto_service.verify_password(pwd, &encoded).await.unwrap();
    assert!(x);
}

#[actix_rt::test]
async fn test_jwt() {
    let crypto_service = CryptoService {
        hash_salt: Arc::new("test_generate_password_hash".to_string()),
        hash_secret: Arc::new("test_generate_password_hash".to_string()),
        jwt_secret: Arc::new("your-256-bit-secret".to_string()),
        access_expires: Arc::new(Duration::minutes(30)),
        refash_expires: Arc::new(Duration::days(7)),
        issuer: Arc::new("test".to_string()),
    };

    let (a, r) = crypto_service.generate_jwt(Uuid::new_v4()).await.unwrap();
    let verify = crypto_service.verify_jwt(a.as_str()).await.is_ok();
    assert!(verify);
    let verify = crypto_service.verify_jwt(r.as_str()).await.is_ok();
    assert!(verify);
}
