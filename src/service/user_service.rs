use crate::entity::tables;
use crate::service::SERVICE;
use rbatis::crud::{CRUD, Skip};
use crate::error::{Result, CustomError, ErrorCode};
use crate::domain::response::user;
use crate::domain::request::user::AddUserDTO;

pub struct UserService {}

impl UserService {
    //用户ID查询
    pub async fn get_user_by_id(&self, user_id: &String) -> Result<Option<tables::User>> {
        let result = SERVICE.rbatis.fetch_by_column(tables::User::id(), user_id).await?;
        Ok(result)
    }

    //ID查询用户,并封装为vo
    pub async fn get_user_vo_by_id(&self, user_id: &String) -> Result<Option<user::UserVo>> {
        let result = self.get_user_by_id(user_id).await?;
        Ok(user::UserVo::from_option(&result))
    }

    //新增用户
    pub async fn add_user(&self, user_dto: &AddUserDTO) -> Result<()> {
        user_dto.check_param()?;
        let user = user_dto.to_user();
        let result = SERVICE.rbatis.save(&user, &[Skip::Column("id"), Skip::Column("create_time")]).await?;
        if result.clone().rows_affected <= 0 {
            tracing::info!("新增用户操作数据库异常{}", &user_dto.username.clone().unwrap());
            return Err(CustomError::from_trait(&ErrorCode::DB_OPERA_ERROR));
        }
        Ok(())
    }
}