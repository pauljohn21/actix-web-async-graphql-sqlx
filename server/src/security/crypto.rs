use anyhow::{Context, Result};
use argon2::Config;
use async_trait::async_trait;
use std::sync::Arc;

pub struct CryptoService {
    pub hash_salt: Arc<String>,
    pub hash_secret: Arc<String>,
    pub jwt_secret: Arc<String>,
}

#[async_trait]
pub trait ExtCryptoService {
    /// 计算密码哈希
    async fn hash_password(&self, password: &str) -> Result<String>;

    /// 验证密码哈希
    async fn verify_password(&self, password: &str, password_hash: &str) -> Result<bool>;
}

#[async_trait]
impl ExtCryptoService for CryptoService {
    /// 计算密码哈希
    async fn hash_password(&self, pwd: &str) -> Result<String> {
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
}

#[actix_rt::test]
async fn test_hash_password() {
    let crypto_service = CryptoService {
        hash_salt: Arc::new("88888888".to_string()),
        hash_secret: Arc::new("88888881".to_string()),
        jwt_secret: Arc::new("88888888".to_string()),
    };

    let pwd = "tiantianxiang";
    let encoded = crypto_service.hash_password(pwd).await.unwrap();
    let x = crypto_service.verify_password(pwd, &encoded).await.unwrap();
    assert!(x)
}
