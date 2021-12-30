use crate::config::config::AppConfig;

//初始化加载日志
pub fn init_log(config: &AppConfig) {
    tracing_subscriber::fmt().with_max_level(get_log_level(config.log_level.as_str())).init();
}




fn get_log_level(level: &str) -> tracing::Level {
    return match level {
        "info" => tracing::Level::INFO,
        "error" => tracing::Level::ERROR,
        "warn" => tracing::Level::WARN,
        "trace" => tracing::Level::TRACE,
        "debug" => tracing::Level::DEBUG,
        _ => tracing::Level::INFO,
    }
}