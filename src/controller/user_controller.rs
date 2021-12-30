use salvo::Request;
use salvo::prelude::*;
use crate::error::{CustomError, ErrorCode};
use crate::domain::response::ResultVo;
use crate::service::SERVICE;
use crate::domain::response::user;
use crate::domain::request;


#[fn_handler]
pub async fn get_user(req: &mut Request, resp: &mut Response) {
    let user_id = req.get_query::<String>("userId");
    if user_id.is_none() {
        resp.render_json(&ResultVo::from_error_trait(&ErrorCode::PARAM_MISS));
        return;
    }
    let result = SERVICE.user_service.get_user_vo_by_id(user_id.as_ref().unwrap()).await;
    resp.render_json(&ResultVo::from_result(&result));
}


#[fn_handler]
pub async fn add_user(req: &mut Request, resp: &mut Response) {
    let user_dto = req.read::<request::user::AddUserDTO>().await;
    if user_dto.is_err() {
        resp.render_json(&ResultVo::from_error_trait(&ErrorCode::REQUEST_PARSING_ERROR));
        return;
    }
    let result = SERVICE.user_service.add_user(&user_dto.unwrap()).await;
    resp.render_json(&ResultVo::from_result(&result));
}