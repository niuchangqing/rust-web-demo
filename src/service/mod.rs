//用户服务
pub mod user_service;

use rbatis::rbatis::Rbatis;
use crate::config::config::AppConfig;
use user_service::*;

pub struct ServiceContext {

    pub rbatis:Rbatis,

    //app配置信息
    pub config:AppConfig,

    //用户服务
    pub user_service: UserService,
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = AppConfig::default();
        //加载日志
        crate::config::log::init_log(&config);
        //加载rbatis
        /// # futures
        let rbatis = futures::executor::block_on({
            crate::mapper::init_rbatis(&config.db_config)
        });
        /// # async_std
        /// let rbatis = async_std::task::block_on(async {
        ///     crate::mapper::init_rbatis(&config.db_config).await
        /// });
        let context = ServiceContext {
            rbatis:rbatis,
            config:config,
            user_service: UserService{},
        };
        context
    }
}

lazy_static! {
    pub static ref SERVICE:ServiceContext = ServiceContext::default();
}