use std::io;
use std::sync::Arc;

use actix_web::{guard, HttpServer};
use actix_web::App;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::{resource, ServiceConfig};
use anyhow::Context;
use guard::{Get, Post};

use gql::ServiceSchema;

use crate::gql::{graphiql, graphql};
use crate::web::gql;
use crate::web::rest::health_check::health_check;
use crate::config::configs::{Configs, DatabaseConfig};

pub mod repository;
pub mod domain;
pub mod web;
pub mod common;
pub mod config;

/// http server application
pub struct Application {
    server: Server,
}

impl Application {
    /// 构建 服务器
    pub async fn build(configs: Arc<Configs>) -> anyhow::Result<Application> {
        // 链接数据库
        let pool = DatabaseConfig::init(&configs.database).await?;

        // 初始化 GraphQL schema.
        let schema = gql::build_schema(pool, &configs.graphql).await;
        log::info!(r#"初始化 "GraphQL Schema" 完成! "#);

        let address = configs.server.get_address();
        let enable = &configs.graphql.graphiql.enable;
        if enable.unwrap_or(false) {
            log::info!("🚀 GraphQL UI: http://{}{}", address, &configs.graphql.graphiql.path);
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
            .wrap(Logger::default())
            .data(configs.clone())
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

    // graphql 入口
    cfg.service(resource(&graphql_config.path)
        .guard(Post()).to(graphql));

    // rest 健康检查
    cfg.service(resource(configs.server.get_health_check())
        .guard(Get()).to(health_check));

    // 开发环境的工具
    let enable = graphql_config.graphiql.enable;
    if enable.unwrap_or(false) {
        cfg.service(resource(&graphql_config.graphiql.path).guard(Get()).to(graphiql));
    }
}
