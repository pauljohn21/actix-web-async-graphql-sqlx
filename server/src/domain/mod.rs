use crate::common::error::errors::AppError;
use anyhow::Context;
use std::collections::HashMap;
use validator::Validate;

pub mod users;

pub trait Validated {
    fn validated(&self) -> anyhow::Result<()>;
}

/// 为任何实现了 validate::Validate d 类型实现 Validated 特征, 提供统一的验证错误处理方法
impl<T: Validate> Validated for &T {
    /// 自定义的验证处理信息
    fn validated(&self) -> anyhow::Result<()> {
        if let Err(errors) = self.validate() {
            // 拿到所有错误的
            let map = errors.field_errors();
            let message = map
                .iter()
                .map(|(column, error_vec)| {
                    return if let Some(error) = error_vec.first() {
                        // 获取字段名(默认) 或者 code(自定义了)
                        let code = &error.code.to_string();
                        let message = &error.message.unwrap_or_default().to_string();
                        format!("[{}] 字段:[{}] [{}] ", code, column, message)
                    } else {
                        String::default()
                    };
                })
                .collect::<String>();
            Err(AppError::RequestParameterError).context(message)?
        }
        Ok(())
    }
}
