use crate::config::db_config::DbConfig;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct AppConfig {
    //数据库连接
    pub db_config:DbConfig,

    //服务端口号
    pub port:String,

    pub log_level:String,
}

impl Default for AppConfig {
    fn default() -> Self {
        let port = std::env::var("PORT").unwrap_or(dotenv::var("PORT").expect("服务端口号不能为空"));
        let log_level = std::env::var("LOG_LEVEL").unwrap_or(dotenv::var("LOG_LEVEL").unwrap_or(String::from("info")));
        AppConfig {
            db_config: DbConfig::default(),
            port,
            log_level: log_level,
        }
    }
}