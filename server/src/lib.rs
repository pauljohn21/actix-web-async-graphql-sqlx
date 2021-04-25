use std::io;
use std::sync::Arc;

use actix_web::{guard, HttpServer, web};
use actix_web::App;
use actix_web::dev::Server;
use actix_web::web::ServiceConfig;
use anyhow::Context;

use gql::ServiceSchema;

use crate::config::configs::{Configs, DatabaseConfig};
use crate::gql::{graphiql, graphql};
use actix_web::middleware::Logger;

pub mod config;
pub mod gql;
pub mod service;
pub mod error;

/// http server application
pub struct Application {
    server: Server,
}

impl Application {
    /// 构建 服务器
    pub async fn build(configs: Arc<Configs>) -> anyhow::Result<Application> {
        let address = configs.server.get_address();
        // 链接数据库
        let pool = DatabaseConfig::init(&configs.database).await?;
        // 初始化 GraphQL schema.
        let schema = gql::build_schema(pool).await;
        let enable = &configs.graphql.graphiql.enable;
        let graphiql_path = &configs.graphql.graphiql.path;
        if enable.unwrap_or(false) {
            log::info!(r#"初始化 "GraphQL Schema" 完成! GraphQL UI: http://{}{}"#, address, graphiql_path);
        }

        let server = build_actix_server(configs, address, schema)?;

        Ok(Application { server })
    }

    /// 启动
    pub async fn run(self) -> anyhow::Result<(), io::Error> {
        self.server.await
    }
}

/// 构建 服务器
fn build_actix_server(configs: Arc<Configs>, address: String, schema: ServiceSchema) -> anyhow::Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
            // TODO: 日志中间件格式调整
            .wrap(Logger::default())
            .data(schema.clone())
            .configure(|cfg| register_service(cfg, configs.clone()))
    })
        .bind(address)
        .context("绑定监听地址失败")?
        .run();
    Ok(server)
}

/// 注册路由 每一个worker都会注册一下
fn register_service(cfg: &mut ServiceConfig, configs: Arc<Configs>) {
    let graphql_config = &configs.graphql;
    cfg.service(web::resource(&graphql_config.path).guard(guard::Post()).to(graphql));
    let enable = graphql_config.graphiql.enable;
    if enable.unwrap_or(false) {
        cfg.service(web::resource(&graphql_config.graphiql.path).guard(guard::Get()).to(graphiql));
    }

    //TODO: 2021-04-22 00:36:25 health_check
}
