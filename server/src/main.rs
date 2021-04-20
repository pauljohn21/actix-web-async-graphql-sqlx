use crate::gql::{graphiql, graphql};
use actix_web::{guard, web, App, HttpServer};
use crate::config::configs::{Configs, LogConfig};

mod config;
mod gql;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // 初始化配置
    let configs = Configs::init_config()?;
    let server_config = configs.server;
    let log_config = configs.log;
    let graphql_config = configs.graphql;

    // 初始日志
    LogConfig::init(&log_config)?;

    log::info!("初始化 配置文件 日志 完成");

    // 初始化 GraphQL schema.
    let schema = gql::build_schema().await;

    let address = server_config.get_address();
    log::info!("初始化 GraphQL schema 完成! GraphQL UI: http://{}{}", &address, &graphql_config.graphiql.path);

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource(&graphql_config.path).guard(guard::Post()).to(graphql))
            .service(web::resource(&graphql_config.graphiql.path).guard(guard::Get()).to(graphiql))
    })
        .bind(&address)?
        .run()
        .await?;
    Ok(())
}
