use axum::{extract::State, routing::options, Router};

use crate::{
    img_user::controller::img_user_routes, response::WebResponse, state::AppState, WebResult,
};
fn preflight() -> Router<AppState> {
    async fn preflight_handler(_: State<AppState>) -> WebResult {
        Ok(WebResponse::ok("Preflight request passed", ()))
    }
    Router::new().route("/", options(preflight_handler))
}

pub fn routes() -> Router<AppState> {
    Router::new().merge(preflight()).merge(img_user_routes())
}
