use std::time::Duration;

use rbatis::db::{DBPoolOptions, DefaultDecoder};
use serde::{Deserialize, Serialize};

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct DbConfig {
    // db host
    pub database_url: String,


    //最大连接数量
    pub max_connections: u32,
    //最小连接数量
    pub min_connections: u32,
    //连接超时时间
    pub connect_timeout: u64,
    // 连接存活时间
    pub max_lifetime: u64,
}

impl Default for DbConfig {
    fn default() -> Self {
        let db_url = std::env::var("DB_DATABASE_URL").unwrap_or(dotenv::var("DB_DATABASE_URL").expect("db database url 不能为空"));

        let max_connections = std::env::var("DB_MAX_CONNECTIONS").unwrap_or(dotenv::var("DB_MAX_CONNECTIONS")
            .unwrap_or("10".to_string())).parse::<u32>().expect("db max connections 格式错误");
        let min_connections = std::env::var("DB_MIN_CONNECTIONS").unwrap_or(dotenv::var("DB_MIN_CONNECTIONS")
            .unwrap_or("0".to_string())).parse::<u32>().expect("db min connections 格式错误");
        DbConfig {
            database_url:db_url,
            max_connections,
            min_connections,
            connect_timeout:60,
            max_lifetime:600,
        }
    }
}

impl DbConfig {
    pub fn to_db_pool(&self) -> DBPoolOptions {
        DBPoolOptions {
            max_connections: self.max_connections.clone(),
            min_connections: self.min_connections.clone(),
            connect_timeout: Duration::from_secs(self.connect_timeout.clone()),
            max_lifetime: Some(Duration::from_secs(self.max_lifetime.clone())),
            idle_timeout: None,
            test_before_acquire: true,
            decoder: Box::new(DefaultDecoder {}),
        }
    }
}