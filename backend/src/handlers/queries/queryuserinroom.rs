use axum::{Json, extract::State, http::HeaderMap};
use serde::Serialize;
use uuid::Uuid;

use crate::{db::{self, user::init::AppState}, helpers::ApiError};

#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub typeofuser: bool
}

#[derive(Serialize)]
pub struct QueryUserRoomResponse {
    users: Vec<User>,
}

pub async fn query_user_in_room_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<QueryUserRoomResponse>, ApiError> {
    let _token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(ApiError::Unauthorized)?;
    let token = headers
        .get("Room-Token")
        .and_then(|v| v.to_str().ok())
        .ok_or(ApiError::Unauthorized)?;

    println!("token: {}", token);

    let room_id = db::user::fetches::getroom(
        &state.db,
        token.to_string(),
    )
    .await
    .ok_or(ApiError::InvalidToken)?;

    let users = db::fetch::fetch::fetch_users(&state.db, room_id)
        .await
        .map_err(|e| {
            println!("Error: {}", e);
            ApiError::Database
        })?;

    Ok(Json(QueryUserRoomResponse { users }))
}
