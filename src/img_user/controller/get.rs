use crate::{
    img_user::model::{request::ImgQueryRequest, response::ImgResposne},
    prisma::{self, img_on_user},
};
use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use prisma_client_rust::query_core::schema_builder::constants::filters;

use crate::{response::WebResponse, state::AppState, WebResult};

// #[utoipa::path(
//   get,
//   tag = "Period",
//   path = "/api/v1/period",
//   params(
//     ("offset" = inline(Option<i64>), Query, description = "Starting point"),
//     ("limit" = inline(Option<i32>), Query, description = "Limit"),
//     ("id" = inline(Option<String>), Query, description = "Period id"),
//     ("name" = inline(Option<String>), Query, description = "Period name"),
//     ("start_date" = inline(Option<i64>), Query, description = "Start time "),
//     ("end_date" = inline(Option<i64>), Query, description = "End time "),
//   ),
//   responses(
//     (
//       status = 200,
//       description = "Get periods",
//       body = Vec<PeriodResponse>,
//       example = json!(
//         {
//           "code": 200,
//           "message": "Get all periods successfully",
//           "data": [
//
//           ],
//           "error": ""
//         }
//       )
//     ),
//   )
// )]
pub fn get_cids() -> Router<AppState> {
    async fn get_cids_handler(
        State(AppState { user_service, .. }): State<AppState>,
        ImgQueryRequest {
            offset,
            limit,
            user_id,
        }: ImgQueryRequest,
    ) -> WebResult {
        let offset = offset.unwrap_or(0);

        let limit = match limit {
            Some(limit) => match limit {
                0..=50 => limit,
                _ => 10,
            },
            None => 10,
        };

        let mut filters = vec![];

        filters.push(img_on_user::user_id::equals(user_id));

        let cids: Vec<ImgResposne> = user_service
            .get_img_user(offset, limit, filters)
            .await?
            .into_iter()
            .map(|u| u.into())
            .collect();
        Ok(WebResponse::ok("Get cids successfully", cids))
    }
    Router::new().route("/", get(get_cids_handler))
}

