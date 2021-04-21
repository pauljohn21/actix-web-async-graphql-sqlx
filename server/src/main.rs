use server::config::configs::{Configs, LogConfig};
use server::Application;
use std::sync::Arc;


#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // 初始化配置
    let configs = Configs::init_config()?;

    // 初始日志
    LogConfig::init(&configs.log)?;

    // 初始化服务器
    let application = Application::build(Arc::new(configs)).await?;

    // 启动服务器
    application.run().await?;
    Ok(())
}
