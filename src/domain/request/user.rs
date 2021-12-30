use serde::{Serialize,Deserialize};
use crate::error::{Result, CustomError, ErrorCode, ErrorTrait};
use crate::entity::{tables, User};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct AddUserDTO {
    // 用户名称
    pub nickname:Option<String>,

    // 用户密码
    pub password:Option<String>,

    // 用户登陆账户
    pub username:Option<String>,
}

impl AddUserDTO {
    pub fn check_param(&self) -> Result<()> {
        if self.username.is_none() {
            return Err(CustomError::from(ErrorCode::PARAM_MISS.get_code(), "[username]不能为空"));
        }
        if self.password.is_none() {
            return Err(CustomError::from(ErrorCode::PARAM_MISS.get_code(), "[password]不能为空"));
        }
        if self.nickname.is_none() {
            return Err(CustomError::from(ErrorCode::PARAM_MISS.get_code(), "[nickname]不能为空"));
        }
        Ok(())
    }

    pub fn to_user(&self) -> tables::User {
        User {
            id: None,
            nickname: self.nickname.clone(),
            password: self.password.clone(),
            username: self.username.clone(),
            create_time: None,
        }
    }
}