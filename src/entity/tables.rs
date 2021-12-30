use serde::{Deserialize, Serialize};
use rbatis::DateTimeNative;

/// 用户表
#[crud_table(table_name:user)]
#[derive(Clone,Debug, Deserialize, Serialize)]
pub struct User {
    // 主键ID
    pub id:Option<i32>,

    // 用户名称
    pub nickname:Option<String>,

    // 用户密码
    pub password:Option<String>,

    // 用户登陆账户
    pub username:Option<String>,

    // 创建时间戳
    // pub create_date: Option<DateTimeNative>,
    pub create_time:Option<DateTimeNative>,
}

impl_field_name_method!(User{id,nickname,password,username,create_time});