use serde::{Deserialize, Serialize};
use crate::entity::{tables, User};

#[derive(Clone,Debug, Deserialize, Serialize)]
pub struct UserVo {
    // 主键ID
    pub id:Option<i32>,

    // 用户名称
    pub nickname:Option<String>,

    // 用户登陆账户
    pub username:Option<String>,

    // 创建时间戳
    // pub create_date: Option<DateTimeNative>,
    pub create_time:Option<i64>,
}

impl From<&tables::User> for UserVo {
    fn from(user: &User) -> Self {
        let mut vo = Self {
            id: user.id.clone(),
            nickname: user.nickname.clone(),
            username: user.username.clone(),
            create_time: None,
        };
        if user.create_time.is_some() {
            vo.create_time = Some(user.create_time.clone().unwrap().timestamp_millis());
        }
        vo
    }
}


impl UserVo {
    pub fn from_option(args: &Option<User>) -> Option<Self> {
        if args.is_none() {
            return None;
        }
        Some(UserVo::from(args.as_ref().unwrap()))
    }
}