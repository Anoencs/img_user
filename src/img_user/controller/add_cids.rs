use axum::{extract::State, routing::post, Router};

use crate::{
    img_user::model::{
        request::{AddImgCollectionRequest, CreateUserCollectionRequest},
        response::UserResponse,
    },
    response::WebResponse,
    state::AppState,
    WebResult,
};

pub fn add_cids_to_collection() -> Router<AppState> {
    async fn add_cids_collection_handler(
        State(AppState { user_service, .. }): State<AppState>,
        AddImgCollectionRequest { collection_id, cid }: AddImgCollectionRequest,
    ) -> WebResult {
        let new_collection: UserResponse = user_service
            .add_img_to_collection(collection_id, cid)
            .await?
            .into();

        Ok(WebResponse::created(
            "Created user img sucessfully",
            UserResponse::from(new_collection),
        ))
    }
    Router::new().route("/add_cid_collection", post(add_cids_collection_handler))
}
