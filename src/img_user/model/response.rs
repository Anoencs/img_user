use crate::prisma::collection;
use serde::{Deserialize, Serialize};

collection::select!(collection_select{
     name
     collection_id
     user_id
     cids
});

pub type UserCollectionSelect = collection_select::Data;

collection::select!(img_select { cids });

pub type ImgSelect = img_select::Data;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub user_id: String,
    pub collection_id: String,
    pub name: String,
}

impl From<UserCollectionSelect> for UserResponse {
    fn from(
        UserCollectionSelect {
            user_id,
            name,
            collection_id,
            cids,
        }: UserCollectionSelect,
    ) -> Self {
        Self {
            user_id,
            name,
            collection_id,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImgResposne {
    pub cids: Vec<String>,
}

impl From<ImgSelect> for ImgResposne {
    fn from(ImgSelect { cids }: ImgSelect) -> Self {
        Self { cids }
    }
}
