use rbatis::rbatis::Rbatis;

use crate::config::db_config::DbConfig;

//初始化rbatis
pub async fn init_rbatis(db_config: &DbConfig) -> Rbatis {
    let mut rbatis = Rbatis::new();
    tracing::info!(">>>>>>>>>> rbatis link database [{}]", &db_config.database_url);
    //rbatis.link("mysql://root:123456@localhost:3306/test");
    // let mysql_connect = rbatis::db::DBConnectOption::from_mysql(&config.db_config.to_mysql_connect()).expect("init db connect failure");
    // rbatis.link_cfg(&mysql_connect, config.db_config.to_db_pool());
    rbatis.link_opt(&db_config.database_url, db_config.to_db_pool()).await.expect("link database failure");
    tracing::info!(">>>>>>>>>> rbatis link database successful");
    rbatis
}