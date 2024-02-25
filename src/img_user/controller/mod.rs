use axum::Router;

use crate::state::AppState;

use self::{create::create_user_img, get::get_cids};

pub mod create;
pub mod get;

pub fn img_user_routes() -> Router<AppState> {
    Router::new().nest(
        "/api/v1/img_user",
        Router::new().merge(get_cids()).merge(create_user_img()),
    )
}
