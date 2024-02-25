use std::sync::Arc;

use crate::{
    error::ErrorResponse,
    prisma::{
        img_on_user::{self, Data, SetParam, WhereParam},
        PrismaClient,
    },
};
use argon2::password_hash::{rand_core::OsRng, SaltString};

use super::model::response::{img_on_user_select, img_select, ImgSelect, UserImgSelect};

#[derive(Clone)]
pub struct UserService {
    pub db: Arc<PrismaClient>,
    salt: SaltString,
}

impl UserService {
    pub fn init(db: &Arc<PrismaClient>) -> Self {
        Self {
            db: db.clone(),
            salt: SaltString::generate(&mut OsRng),
        }
    }

    pub async fn create_img_user(
        &self,
        user_id: String,
        cid: String,
        params: Vec<SetParam>,
    ) -> Result<UserImgSelect, ErrorResponse> {
        self.db
            .img_on_user()
            .create(user_id, cid, params)
            .select(img_on_user_select::select())
            .exec()
            .await
            .map_err(Into::into)
    }

    pub async fn get_img_user(
        &self,
        offset: i64,
        limit: i64,
        filters: Vec<WhereParam>,
    ) -> Result<Vec<ImgSelect>, ErrorResponse> {
        let cids = self
            .db
            .img_on_user()
            .find_many(filters)
            .skip(offset)
            .take(limit)
            .select(img_select::select())
            .exec()
            .await?;

        Ok(cids)
    }
}
