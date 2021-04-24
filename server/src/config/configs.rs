use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use std::env::current_dir;
use anyhow::Context;
use std::path::PathBuf;
use sqlx::{Postgres, Pool, ConnectOptions};
use sqlx::postgres::{PgPoolOptions, PgConnectOptions};
use std::time::Duration;
use log::LevelFilter;

/// 配置文件目录
pub const CONFIG_PATH: &str = "config/";
pub const SERVER_CONFIG_PATH: &str = "server/config/";

/// 配置文件默认文件
pub const DEFAULT_CONFIG: &str = "base";

/// 配置环境标识
pub const SERVER_ENVIRONMENT: &str = "SERVER_ENVIRONMENT";

/// 环境变量覆盖配置文件前缀
pub const SERVER_PREFIX: &str = "SERVER";

/// 环境变量覆盖配置文件分隔符
pub const SEPARATOR: &str = "_";

/// 配置项结构体
#[derive(Deserialize, Clone, Debug)]
pub struct Configs {
    pub server: ServerConfig,
    pub graphql: GraphQlConfig,
    pub database: DatabaseConfig,
    pub log: LogConfig,
}

/// 服务配置
#[derive(Deserialize, Clone, Debug)]
pub struct ServerConfig {
    pub name: String,
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub context_path: Option<String>,
    pub health_check: Option<String>,
}

/// Graphql配置
#[derive(Deserialize, Clone, Debug)]
pub struct GraphQlConfig {
    pub path: String,
    pub graphiql: GraphiQlConfig,
}

/// Graphiql配置
#[derive(Deserialize, Clone, Debug)]
pub struct GraphiQlConfig {
    pub path: String,
    pub enable: Option<bool>,
}

/// 数据库配置
#[derive(Deserialize, Clone, Debug)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

/// 日志相关配置
#[derive(Deserialize, Clone, Debug)]
pub struct LogConfig {
    /// 日志配置文件
    pub file: String,
}

impl Configs {
    /// 初始化配置文件
    pub fn init_config() -> anyhow::Result<Configs> {
        // 加载环境变量
        dotenv::dotenv().ok();

        // 加载配置文件
        let mut settings = config::Config::default();

        let config_dir = get_config_dir()?;

        settings
            .merge(config::File::from(config_dir.join(DEFAULT_CONFIG)))
            .context(format!("加载默认配置文件:[{}] 失败!", DEFAULT_CONFIG))?;

        // 读取当前环境标志
        let environment = dotenv::var(SERVER_ENVIRONMENT)
            .context(format!("读取当前环境标志:[{}] 失败!", SERVER_ENVIRONMENT))?;

        settings
            .merge(config::File::from(config_dir.join(&environment)))
            .context(format!("加载自定义配置文件:[{}] 失败!", &environment))?;

        // 从环境变量或.env中添加设置（以APP前缀和'__'作为分隔符）
        // APP_SERVER_PORT = 5001 将覆盖 ApplicationConfig.server.port
        settings.merge(config::Environment::with_prefix(SERVER_PREFIX).separator(SEPARATOR))?;

        // 将读取的配置文件转换为配置文件结构体
        let config = settings.try_into().context("配置文件转换错误!")?;

        Ok(config)
    }
}

impl ServerConfig {
    /// 获取服务地址
    pub fn get_address(&self) -> String {
        format!("{}:{}", &self.host, &self.port)
    }
}

impl LogConfig {
    /// 初始化日志配置
    pub fn init(config: &LogConfig) -> anyhow::Result<()> {
        let config_dir = get_config_dir()?;
        let result = log4rs::init_file(config_dir.join(&config.file), Default::default())
            .context(format!("初始化日志配置:[{}]失败!", &config.file));
        log::info!(r#"初始化 "配置文件" "日志" 完成"#);
        result
    }
}

impl DatabaseConfig {
    /// 初始化数据库连接池
    pub async fn init(config: &DatabaseConfig) -> anyhow::Result<Pool<Postgres>> {
        let mut options = PgConnectOptions::new()
            .username(&config.username)
            .password(&config.password)
            .host(&config.host)
            .port(config.port)
            .database(&config.database_name);
        // 设置 sql 日志级别
        options.log_statements(LevelFilter::Debug);
        let pool = PgPoolOptions::new()
            .connect_timeout(Duration::from_secs(2))
            .connect_with(options).await?;
        Ok(pool)
    }
}


/// 获取配置文件路径
fn get_config_dir() -> anyhow::Result<PathBuf> {
    let base_path = current_dir().context("无法确定当前目录")?;

    let mut config_dir = base_path.join(CONFIG_PATH);

    if !config_dir.as_path().exists() {
        config_dir = base_path.join(SERVER_CONFIG_PATH);
    };
    Ok(config_dir)
}
