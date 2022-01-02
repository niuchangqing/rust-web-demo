use salvo::Request;
use salvo::prelude::*;
use crate::error::{CustomError, ErrorCode};
use crate::domain::response::ResultVo;
use crate::service::SERVICE;
use crate::domain::response::user;
use crate::domain::request;
use crate::error::Result;


#[fn_handler]
pub async fn get_user(req: &mut Request, resp: &mut Response) -> Result<ResultVo<Option<user::UserVo>>> {
    let user_id = req.get_query::<String>("userId");
    if user_id.is_none() {
        return Err(CustomError::from_trait(&ErrorCode::PARAM_MISS));
    }
    let result = SERVICE.user_service.get_user_vo_by_id(user_id.as_ref().unwrap()).await;
    Ok(ResultVo::from_result(&result))
}


#[fn_handler]
pub async fn add_user(req: &mut Request, resp: &mut Response) -> Result<ResultVo<()>> {
    let user_dto = req.read::<request::user::AddUserDTO>().await;
    if user_dto.is_err() {
        return Err(CustomError::from_trait(&ErrorCode::REQUEST_PARSING_ERROR));
    }
    let result = SERVICE.user_service.add_user(&user_dto.unwrap()).await;
    Ok(ResultVo::from_result(&result))
}

#[fn_handler]
pub async fn test_error(resp: &mut Response) -> Result<ResultVo<String>> {
    // let f = std::fs::File::open("/Users/xuweihong/logs/aaa.jpeg")?;
    // Err(CustomError::from_trait(&ErrorCode::DB_OPERA_ERROR))
    Ok(ResultVo::from(&String::from("成功")))
}