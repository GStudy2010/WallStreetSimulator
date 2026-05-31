use axum::{Json, extract::State, http::{HeaderMap, StatusCode}, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::db::{self, user::init::AppState};

#[derive(Deserialize)]
pub struct JoinRoomRequest {
    name: String,
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
    let room_id = match db::room::fetch::fetchroomidbyname(&state.db, payload.name.clone()).await {
        Ok(s) => {
            s
        } 
        Err(e) => {
            println!("Error while fetching: {}", e);
            let resp = JoinRoomResponse {
                message: "Error while fetching from database".to_string()
            };
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(resp));
        }
    };
    // if public
    if payload.pop {
        match db::room::joinroomdb::joinroomdb(&state.db, user_id, room_id).await {
            Ok(()) => {
                let resp = JoinRoomResponse {
                    message: "Room created".to_string()
                };
                (StatusCode::CREATED, Json(resp))
            }
            Err(e) => {
                println!("Error while creating a room in db: {}", e);
                let resp = JoinRoomResponse {
                    message: "Internal server error".to_string()
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(resp))
            }
        }
    } else {
        match db::room::fetch::fetchpasswordbyname(&state.db, payload.name).await {
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
                    message: "Room created".to_string()
                };
                (StatusCode::CREATED, Json(resp))
            }
            Err(e) => {
                println!("Error while creating a room in db: {}", e);
                let resp = JoinRoomResponse {
                    message: "Internal server error".to_string()
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(resp))
            }
        }
    }
}
