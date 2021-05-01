use actix_web::rt::time::Instant;
use anyhow::Context;
use server::config::configs::Configs;
use server::Application;
use std::sync::Arc;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let instant = Instant::now();
    // 初始化配置
    let configs = Configs::init_config()?;

    // 初始日志
    LogTracer::init().expect("Failed to set logger");
    // LogConfig::init(&configs.log)?;
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("trace"));

    let subscriber = Registry::default().with(env_filter).with(Layer::default());

    set_global_default(subscriber).context("Failed to set subscriber")?;

    // 初始化服务器
    let application = Application::build(Arc::new(configs)).await?;

    tracing::info!("🎉 Started Application in {:.3?} ", instant.elapsed());
    // 启动服务器
    application.run().await?;
    Ok(())
}
