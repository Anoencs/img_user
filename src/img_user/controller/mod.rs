use axum::Router;

use crate::state::AppState;

use self::{
    add_cids::add_cids_to_collection,
    create::create_user_collection,
    get::{get_cids, get_colelction},
};

pub mod add_cids;
pub mod create;
pub mod get;

pub fn img_user_routes() -> Router<AppState> {
    Router::new().nest(
        "/api/v1/img_user",
        Router::new()
            .merge(get_cids())
            .merge(create_user_collection())
            .merge(add_cids_to_collection())
            .merge(get_colelction()),
    )
}
