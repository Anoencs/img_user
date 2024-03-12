use crate::{
    img_user::model::{
        request::ImgQueryRequest,
        response::{ImgResposne, UserCollectionSelect},
    },
    prisma::{self, collection},
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
            collection_id,
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

        filters.push(collection::collection_id::equals(collection_id));

        let cids: Vec<ImgResposne> = user_service
            .get_img_collection(offset, limit, filters)
            .await?
            .into_iter()
            .map(|u| u.into())
            .collect();
        Ok(WebResponse::ok("Get cids successfully", cids))
    }
    Router::new().route("/", get(get_cids_handler))
}

pub fn get_colelction() -> Router<AppState> {
    async fn get_collection_handler(
        State(AppState { user_service, .. }): State<AppState>,
        Path(user_id): Path<String>,
    ) -> WebResult {
        let collection: UserCollectionSelect = user_service.get_collection(user_id).await?;
        Ok(WebResponse::ok("Get cids successfully", collection))
    }
    Router::new().route("/:user_id", get(get_collection_handler))
}
