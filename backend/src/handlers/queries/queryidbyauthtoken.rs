use axum::{Json, extract::State, http::HeaderMap};
use serde::Serialize;
use uuid::Uuid;

use crate::{db::{self, user::init::AppState}, helpers::ApiError};


#[derive(Serialize)]
pub struct QueryRoomResponse {
    id: Uuid,
}

pub async fn queryidbyauthtoken(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<QueryRoomResponse>, ApiError> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(ApiError::Unauthorized)?;

    let user_id = db::user::fetches::getsession(
        &state.db,
        token.to_string(),
    )
    .await
    .ok_or(ApiError::InvalidToken)?;

    Ok(Json(QueryRoomResponse { id: user_id }))
}
