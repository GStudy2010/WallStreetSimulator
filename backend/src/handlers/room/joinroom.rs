use axum::{Json, extract::State, http::{HeaderMap, StatusCode}, response::IntoResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::{self, user::init::AppState};

#[derive(Deserialize)]
pub struct JoinRoomRequest {
    room_id: Uuid,
}
#[derive(Serialize)]
pub struct JoinRoomResponse {
    message: String,
}

pub async fn join_room_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<JoinRoomRequest>
) -> impl IntoResponse{
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));
    let Some(token) = token else {
        let resp = JoinRoomResponse {
            message: "No token provided".to_string()
        };
        return (StatusCode::UNAUTHORIZED, Json(resp));
    };
    let Some(user_id) = db::user::fetches::getsession(&state.db, token.to_string()).await else {
        let resp = JoinRoomResponse {
            message: "Invalid token".to_string()
        };
        return (StatusCode::BAD_REQUEST, Json(resp));
    };
    match db::room::joinroomdb::joinroom(&state.db, payload.room_id, user_id).await {
        Ok(Some(token)) => {
            let resp = JoinRoomResponse {
                message: token,
            };
            (StatusCode::OK, Json(resp))
        }
        Ok(None) => {
            let resp = JoinRoomResponse {
                message: "Room full".to_string(),
            };
            (StatusCode::FORBIDDEN, Json(resp))
        }
        Err(e) => {
            println!("Error while joining a room: {}", e);
            let resp = JoinRoomResponse {
                message: "Failed to join".to_string(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(resp))
        }
    }
}
