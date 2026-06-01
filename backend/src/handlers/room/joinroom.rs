use axum::{Json, extract::State, http::{HeaderMap, StatusCode}, response::IntoResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::{self, user::init::AppState};

#[derive(Deserialize)]
pub struct JoinRoomRequest {
    id: Uuid,
    pop: bool,
    password: String, // empty string for no password
}

#[derive(Serialize)]
pub struct JoinRoomResponse {
    message: String,
}

pub async fn join_room_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<JoinRoomRequest>
) -> impl IntoResponse {
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
    let room_id = payload.id;
    // if public
    if payload.pop {
        match db::room::joinroomdb::joinroomdb(&state.db, room_id, user_id).await {
            Ok(()) => {
                let resp = JoinRoomResponse {
                    message: "Room joined".to_string()
                };
                (StatusCode::OK, Json(resp))
            }
            Err(e) => {
                println!("Error while joining a room in db: {}", e);
                let resp = JoinRoomResponse {
                    message: "Internal server error".to_string()
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(resp))
            }
        }
    } else {
        match db::room::fetch::fetchpasswordbyname(&state.db, payload.id).await {
            Ok(s) => {
                if payload.password == s {
                    let resp = JoinRoomResponse {
                        message: "Invalid password".to_string()
                    };
                    return (StatusCode::UNAUTHORIZED, Json(resp));
                }
            } 
            Err(e) => {
                println!("Error while fetching: {}", e);
                let resp = JoinRoomResponse {
                    message: "Error while fetching from database".to_string()
                };
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(resp));
            }
        }
        match db::room::joinroomdb::joinroomdb(&state.db, user_id, room_id).await {
            Ok(()) => {
                let resp = JoinRoomResponse {
                    message: "Room joined".to_string()
                };
                (StatusCode::OK, Json(resp))
            }
            Err(e) => {
                println!("Error while joining a room in db: {}", e);
                let resp = JoinRoomResponse {
                    message: "Internal server error".to_string()
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(resp))
            }
        }
    }
}
