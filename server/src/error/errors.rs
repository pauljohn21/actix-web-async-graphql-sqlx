use async_graphql::{Error, ErrorExtensions};
use thiserror::Error;

/// 定义错误枚举
#[derive(Debug, Error)]
pub enum AppError {
    #[error("服务器内部错误")]
    InternalError,

    #[error("客户端错误")]
    ClientError,

    #[error("用户名已存在")]
    UsernameAlreadyExists,
}

impl AppError {
    // TODO: 2021-04-25 00:16:38 错误处理先这样吧 以后有了更好的再处理 总归服务器的错误不应该暴露到客户端去.
    // TODO: 在返回给客户端的新增中新增了 code 业务状态码, 作为业务状态梳理
    /// 返回错误扩展并输出日志的闭包
    pub fn extend_log(self) -> Box<dyn FnOnce(anyhow::Error) -> async_graphql::Error> {
        Box::new(move |error| {
            log::error!("{:?}", error);
            self.extend()
        })
    }
}

/// 实现错误扩展
impl ErrorExtensions for AppError {
    /// 定义基本扩展
    fn extend(&self) -> Error {
        self.extend_with(|error, e| {
            match error {
                AppError::InternalError => e.set("code", "B0001"),
                AppError::ClientError => e.set("code", "A0001"),
                AppError::UsernameAlreadyExists => e.set("code", "A0002"),
            }
        })
    }
}

