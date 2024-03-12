use axum::{extract::State, routing::post, Router};

use crate::{
    img_user::model::{request::CreateUserCollectionRequest, response::UserResponse},
    response::WebResponse,
    state::AppState,
    WebResult,
};

// #[utoipa::path(
//   post,
//   tag = "Period",
//   path = "/api/v1/period/create",
//   request_body(
//     content = CreatePeriodRequest,
//     description = "Create Period Request",
//   ),
//   responses(
//     (
//       status = 201,
//       description = "Period created",
//       body = PeriodResponse,
//       example = json! (
//         {
//           "code": 201,
//           "message": "Created new period successfully",
//           "data": {
//             "createdAt": 1696932804946_i64,
//             "updatedAt": 1696932804946_i64
//           },
//           "error": ""
//         }
//       )
//     ),
//   )
// )]
pub fn create_user_collection() -> Router<AppState> {
    async fn create_user_collection_handler(
        State(AppState { user_service, .. }): State<AppState>,
        CreateUserCollectionRequest {
            user_id,
            name_collection,
        }: CreateUserCollectionRequest,
    ) -> WebResult {
        let mut params = vec![];

        let new_user_collection: UserResponse = user_service
            .create_collection(user_id, name_collection, params)
            .await?
            .into();

        Ok(WebResponse::created(
            "Created user img sucessfully",
            UserResponse::from(new_user_collection),
        ))
    }
    Router::new().route("/create", post(create_user_collection_handler))
}
