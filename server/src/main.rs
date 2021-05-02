use std::env::current_dir;
use std::io;
use std::sync::Arc;

use actix_web::rt::time::Instant;
use anyhow::Context;
use async_graphql::futures_util::TryFutureExt;
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, EnvFilter};

use server::config::configs::{get_config_dir, Configs};
use server::Application;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let instant = Instant::now();
    // 初始化配置
    let configs = Configs::init_config()?;

    // 初始日志
    LogTracer::init().expect("Failed to set logger");
    let dir = current_dir().context("无法确定当前目录")?.join("log");
    let file_appender = tracing_appender::rolling::daily(dir, "server.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // let subscriber = tracing_subscriber::registry()
    //     .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
    //     .with(fmt::Layer::new().with_writer(io::stdout))
    //     .with(fmt::Layer::new().with_writer(non_blocking));
    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_bunyan_formatter::JsonStorageLayer)
        .with(tracing_bunyan_formatter::BunyanFormattingLayer::new(
            "server".to_string(),
            || io::stdout(),
        ))
        .with(tracing_bunyan_formatter::BunyanFormattingLayer::new(
            "server".to_string(),
            non_blocking,
        ));
    tracing::subscriber::set_global_default(subscriber).expect("Unable to set a global collector");
    tracing::info!("Loading tracing and log ");
    // 初始化服务器
    let application = Application::build(Arc::new(configs)).await?;

    tracing::info!("🎉 Started Application in {:.3?} ", instant.elapsed());
    // 启动服务器
    application.run().await?;
    Ok(())
}
