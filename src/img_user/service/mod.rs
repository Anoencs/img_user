use std::sync::Arc;

use crate::{
    error::ErrorResponse,
    prisma::{
        collection::{self, collection_id, Data, SetParam, WhereParam},
        PrismaClient,
    },
};
use argon2::password_hash::{rand_core::OsRng, SaltString};

use super::model::response::{collection_select, img_select, ImgSelect, UserCollectionSelect};
use crate::helpers::id::generate_id;

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

    pub async fn create_collection(
        &self,
        user_id: String,
        name: String,
        params: Vec<SetParam>,
    ) -> Result<UserCollectionSelect, ErrorResponse> {
        self.db
            .collection()
            .create(generate_id(15), name, user_id, params)
            .select(collection_select::select())
            .exec()
            .await
            .map_err(Into::into)
    }

    pub async fn add_img_to_collection(
        &self,
        collection_id: String,
        cid: String,
    ) -> Result<UserCollectionSelect, ErrorResponse> {
        let mut cids: Vec<String> = self
            .db
            .collection()
            .find_first(vec![collection::collection_id::equals(
                collection_id.clone(),
            )])
            .select(img_select::select())
            .exec()
            .await?
            .unwrap()
            .cids;

        cids.push(cid);

        self.db
            .collection()
            .update(
                collection::collection_id::equals(collection_id),
                vec![collection::cids::set(cids)],
            )
            .select(collection_select::select())
            .exec()
            .await
            .map_err(Into::into)
    }

    pub async fn get_img_collection(
        &self,
        offset: i64,
        limit: i64,
        filters: Vec<WhereParam>,
    ) -> Result<Vec<ImgSelect>, ErrorResponse> {
        let cids = self
            .db
            .collection()
            .find_many(filters)
            .skip(offset)
            .take(limit)
            .select(img_select::select())
            .exec()
            .await?;

        Ok(cids)
    }

    pub async fn get_collection(
        &self,
        user_id: String,
    ) -> Result<UserCollectionSelect, ErrorResponse> {
        let collection = self
            .db
            .collection()
            .find_unique(collection::user_id::equals(user_id))
            .select(collection_select::select())
            .exec()
            .await?
            .unwrap();

        Ok(collection)
    }
}
