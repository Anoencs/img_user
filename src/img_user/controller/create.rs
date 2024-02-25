use axum::{extract::State, routing::post, Router};

use crate::{
    img_user::model::{request::CreateUserImgRequest, response::UserResponse},
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
pub fn create_user_img() -> Router<AppState> {
    async fn create_user_img_handler(
        State(AppState { user_service, .. }): State<AppState>,
        CreateUserImgRequest { user_id, cid }: CreateUserImgRequest,
    ) -> WebResult {
        let mut params = vec![];

        let new_user_img: UserResponse = user_service
            .create_img_user(user_id, cid, params)
            .await?
            .into();

        Ok(WebResponse::created(
            "Created user img sucessfully",
            UserResponse::from(new_user_img),
        ))
    }
    Router::new().route("/create", post(create_user_img_handler))
}
