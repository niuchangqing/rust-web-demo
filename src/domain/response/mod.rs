use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use crate::error::{CustomError, ErrorCode, ErrorTrait};

pub mod user;

#[derive(Debug,Clone,Deserialize, Serialize)]
pub struct ResultVo<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

pub const SUCCESS_MSG: &str = "成功";

pub const SUCCESS_CODE: i32 = 0;


impl <T> ResultVo<T> where T: DeserializeOwned + Serialize + Clone {
    pub fn from_result(result: &Result<T, CustomError>) -> Self {
        if result.is_ok() {
            return Self {
                code: SUCCESS_CODE,
                message: SUCCESS_MSG.to_string(),
                data: result.clone().ok(),
            }
        } else {
            let error = result.clone().err().unwrap();
            return Self {
                code: error.code,
                message: error.message,
                data: None,
            }
        }
    }

    pub fn from_none() -> Self {
        return Self {
            code: SUCCESS_CODE,
            message: SUCCESS_MSG.to_string(),
            data: None
        }
    }

    pub fn from_data(data: Option<T>) -> Self {
        return Self {
            code: SUCCESS_CODE,
            message: SUCCESS_MSG.to_string(),
            data: data.clone(),
        }
    }

    pub fn from(data: &T) -> Self<> {
        return Self {
            code: SUCCESS_CODE,
            message: SUCCESS_MSG.to_string(),
            data: Some(data.clone()),
        }
    }
}

impl ResultVo<String> {
    pub fn from_error(code: i32, message: &str) -> Self {
        return Self {
            code: code,
            message: message.to_string(),
            data: None,
        }
    }

    pub fn from_error_trait(error: &dyn ErrorTrait) -> CustomError {
        error.to_custom_error()
    }
}

