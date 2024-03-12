use axum::{
    async_trait,
    body::Body,
    extract::{FromRequest, FromRequestParts, Query},
    http::request::Parts,
    http::Request,
    Json,
};
use serde::Deserialize;
use validator::Validate;

use crate::{error::ErrorResponse, helpers::validation::validation_message, state::AppState};

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserCollectionRequest {
    pub user_id: String,
    pub name_collection: String,
}

#[async_trait]
impl FromRequest<AppState, Body> for CreateUserCollectionRequest {
    type Rejection = ErrorResponse;

    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<CreateUserCollectionRequest>::from_request(req, state).await?;

        let CreateUserCollectionRequest {
            user_id,
            name_collection,
        } = &body;

        Ok(body)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImgQueryRequest {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub collection_id: String,
}

#[async_trait]
impl FromRequestParts<AppState> for ImgQueryRequest {
    type Rejection = ErrorResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Query(img_query) = Query::<ImgQueryRequest>::from_request_parts(parts, state).await?;

        Ok(img_query)
    }
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct AddImgCollectionRequest {
    pub collection_id: String,
    pub cid: String,
}

#[async_trait]
impl FromRequest<AppState, Body> for AddImgCollectionRequest {
    type Rejection = ErrorResponse;

    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<AddImgCollectionRequest>::from_request(req, state).await?;

        let AddImgCollectionRequest { collection_id, cid } = &body;

        Ok(body)
    }
}
