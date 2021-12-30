
use serde::Serialize;

pub type Result<T> = std::result::Result<T, CustomError>;


#[derive(Debug, Serialize, Clone)]
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

pub trait ErrorTrait {
    fn get_code(&self) -> i32;

    fn to_custom_error(&self) -> CustomError;
}

#[derive(Debug)]
pub enum ErrorCode {
    PARAM_MISS,
    DB_OPERA_ERROR,
    REQUEST_PARSING_ERROR,
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
            _ => CustomError::from(500, "系统异常"),
        }
    }
}