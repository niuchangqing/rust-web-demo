use std::fmt::{Display, Formatter};

use salvo::{Depot, Request, Response};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

use crate::domain::response::ResultVo;
use rbatis::Error;

pub type Result<T> = std::result::Result<T, CustomError>;


#[derive(Debug, Serialize, Clone,Deserialize)]
pub struct CustomError {
    pub code: i32,
    pub message: String,
}

impl CustomError {
    pub fn from(code: i32, message: &str) -> Self {
        CustomError {
            code: code,
            message: message.to_string(),
        }
    }

    pub fn from_trait(error: &dyn ErrorTrait) -> Self {
        error.to_custom_error()
    }
}
//响应处理
#[async_trait]
impl salvo::Writer for CustomError {
    async fn write(mut self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        let result = crate::domain::response::ResultVo::from_error(self.code, &self.message.clone());
        res.render_json(&result)
    }
}
#[async_trait]
impl <T: serde::Serialize + std::marker::Send> salvo::Writer for ResultVo<T> {
    async fn write(mut self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        res.render_json(&self)
    }
}

pub trait ErrorTrait {
    fn get_code(&self) -> i32;

    fn to_custom_error(&self) -> CustomError;
}

#[derive(Debug)]
pub enum ErrorCode {
    PARAM_MISS,
    DB_OPERA_ERROR,
    REQUEST_PARSING_ERROR,
    SYSTEM_ERROR,
}

// impl From<ErrorCode> for CustomError {
//     fn from(args: ErrorCode) -> Self {
//         Self::from_trait(&ErrorCode::SYSTEM_ERROR)
//     }
// }
//错误信息格式化
impl From<&dyn std::error::Error> for CustomError {
    fn from(args: &dyn std::error::Error) -> Self {
        Self::from_trait(&ErrorCode::SYSTEM_ERROR)
    }
}

impl From<std::io::Error> for CustomError {
    fn from(_: std::io::Error) -> Self {
        Self::from_trait(&ErrorCode::SYSTEM_ERROR)
    }
}

impl From<rbatis::Error> for CustomError {
    fn from(_: rbatis::Error) -> Self {
        Self::from_trait(&ErrorCode::DB_OPERA_ERROR)
    }
}

impl ErrorTrait for ErrorCode {

    fn get_code(&self) -> i32 {
        self.to_custom_error().code
    }

    fn to_custom_error(&self) -> CustomError {
        return match self {
            ErrorCode::PARAM_MISS => CustomError::from(1000, "参数错误"),
            ErrorCode::DB_OPERA_ERROR => CustomError::from(1001, "DB操作失败"),
            ErrorCode::REQUEST_PARSING_ERROR => CustomError::from(1002, "请求参数解析失败"),
            ErrorCode::SYSTEM_ERROR => CustomError::from(500, "系统异常"),
            _ => CustomError::from(500, "系统异常"),
        }
    }
}