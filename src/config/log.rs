use crate::config::config::AppConfig;
use tracing_subscriber::fmt::writer::MakeWriterExt;

//初始化加载日志
pub fn init_log(config: &AppConfig) {
    // tracing_subscriber::fmt()
    //     .with_thread_ids(true)//是否输出线程ID
    //     .with_target(false)//是否打印输出日志所在包路径
    //     // .with_thread_names(true)//是否打印线程名称
    //     .with_max_level(get_log_level(config.log_level.as_str()))
    //     .init();
    fast_log::init_split_log(
        "/Users/xuweihong/logs/requests.log",
        fast_log::consts::LogSize::MB(1),
        fast_log::plugin::file_split::RollingType::KeepTime(std::time::Duration::from_secs(24 * 3600)),
        log::Level::Info,
        None,
        Box::new(fast_log::plugin::packer::ZipPacker{}),
        true,
    );
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