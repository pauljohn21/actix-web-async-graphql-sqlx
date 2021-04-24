use async_graphql::{Error, ErrorExtensions};
use thiserror::Error;

/// 定义错误枚举
#[derive(Debug, Error)]
pub enum AppError {
    #[error("服务器内部错误")]
    ServerInternalError,

    #[error("客户端错误")]
    ClientError,

    #[error("用户名已存在")]
    UsernameAlreadyExists,
}


/// 实现错误扩展
impl ErrorExtensions for AppError {
    /// 定义基本扩展
    fn extend(&self) -> Error {
        self.extend_with(|error, e| {
            match error {
                AppError::ServerInternalError => e.set("B0001", "ServerInternalError"),
                AppError::ClientError => e.set("A0001", "ServerInternalError"),
                AppError::UsernameAlreadyExists => e.set("A0002", "UsernameAlreadyExists"),
            }
        })
    }
}

