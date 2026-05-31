use axum::{Json, extract::State, http::HeaderMap};
use serde::Serialize;

use crate::{db::{self, user::init::AppState}, helpers::ApiError};

#[derive(Serialize)]
pub struct Room {
    pub amount_players: u32,
    pub start_cash: f64,
    pub years: u32
}

#[derive(Serialize)]
pub struct QueryRoomResponse {
    rooms: Vec<Room>,
}

pub async fn query_room_handler(
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

    let rooms = db::fetch::fetch::fetch_rooms(&state.db, user_id)
        .await
        .map_err(|_| ApiError::Database)?;

    Ok(Json(QueryRoomResponse { rooms }))
}
