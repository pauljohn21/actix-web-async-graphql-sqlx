use std::io;
use std::sync::Arc;

use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::{resource, ServiceConfig};
use actix_web::App;
use actix_web::{guard, HttpServer};
use anyhow::Context;
use async_graphql::Context as GraphQLContext;
use guard::{Get, Post};
use sqlx::{PgPool, Pool, Postgres};

use gql::ServiceSchema;

use crate::config::configs::{Configs, CryptoConfig, DatabaseConfig};
use crate::gql::{graphiql, graphql};
use crate::security::crypto::CryptoService;
use crate::web::gql;
use crate::web::gql::GraphqlResult;
use crate::web::rest::health_check::health_check;

pub mod common;
pub mod config;
pub mod domain;
pub mod repository;
pub mod security;
pub mod service;
pub mod web;

/// 全局的 state
pub struct State {
    // 数据库连接池
    pool: Arc<PgPool>,
    // 加密服务
    crypto: Arc<CryptoService>,
}

impl State {
    // 通过 GraphQLContext 获取 数据库连接池
    pub fn get_pool(ctx: &GraphQLContext<'_>) -> GraphqlResult<Arc<Pool<Postgres>>> {
        Ok(ctx.data::<Arc<State>>()?.pool.clone())
    }

    // 通过 GraphQLContext 获取 加密服务
    pub fn get_crypto_server(ctx: &GraphQLContext<'_>) -> GraphqlResult<Arc<CryptoService>> {
        Ok(ctx.data::<Arc<State>>()?.crypto.clone())
    }
}

/// http server application
pub struct Application {
    server: Server,
}

impl Application {
    /// 构建 服务器
    pub async fn build(configs: Arc<Configs>) -> anyhow::Result<Application> {
        // 链接数据库
        let pool = Arc::new(DatabaseConfig::init(&configs.database).await?);
        let crypto = Arc::new(CryptoConfig::get_crypto_server(&configs.crypto));
        let state = Arc::new(State { pool, crypto });
        // 初始化 GraphQL schema.
        let schema = gql::build_schema(state.clone(), &configs.graphql).await;
        log::info!(r#"初始化 'GraphQL Schema' 完成! "#);

        let address = configs.server.get_address();
        let enable = &configs.graphql.graphiql.enable;
        if enable.unwrap_or(false) {
            log::info!(
                "🚀GraphQL UI: http://{}{} 🚀",
                address,
                &configs.graphql.graphiql.path
            );
        }

        let server = build_actix_server(configs, address, state, schema)?;

        Ok(Application { server })
    }

    /// 启动
    pub async fn run(self) -> anyhow::Result<(), io::Error> {
        self.server.await
    }
}

/// 构建 服务器
fn build_actix_server(
    configs: Arc<Configs>,
    address: String,
    state: Arc<State>,
    schema: ServiceSchema,
) -> anyhow::Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(configs.clone())
            .data(state.clone())
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
    cfg.service(resource(&graphql_config.path).guard(Post()).to(graphql));

    // rest 健康检查
    cfg.service(
        resource(configs.server.get_health_check())
            .guard(Get())
            .to(health_check),
    );

    // 开发环境的工具
    let enable = graphql_config.graphiql.enable;
    if enable.unwrap_or(false) {
        cfg.service(
            resource(&graphql_config.graphiql.path)
                .guard(Get())
                .to(graphiql),
        );
    }
}
