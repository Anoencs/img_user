use crate::prisma::img_on_user;
use serde::{Deserialize, Serialize};

img_on_user::select!(img_on_user_select{
     user_id
     cid
});

pub type UserImgSelect = img_on_user_select::Data;

img_on_user::select!(img_select { cid });

pub type ImgSelect = img_select::Data;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub user_id: String,
    pub cid: String,
}

impl From<UserImgSelect> for UserResponse {
    fn from(
        UserImgSelect{user_id, cid}: UserImgSelect
    ) -> Self {
        Self{user_id, cid}
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImgResposne {pub cid: String}

impl From<ImgSelect> for ImgResposne {
    fn from(ImgSelect{cid}: ImgSelect) -> Self {
        Self{cid}
    }
}
