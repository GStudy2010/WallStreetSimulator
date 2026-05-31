use axum::{Json, extract::State, http::{HeaderMap, StatusCode}, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::db::{self, user::init::AppState};

#[derive(Deserialize)]
pub struct CreateRoomRequest {
    name: String,
    pop: bool,
    all_players: u32,
    start_cash: f64,
    years: u32,
    password: String, // empty string for no password
}

#[derive(Serialize)]
pub struct CreateRoomResponse {
    message: String,
}

pub async fn create_room_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateRoomRequest>
) -> impl IntoResponse {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));
    let Some(token) = token else {
        let resp = CreateRoomResponse {
            message: "No token provided".to_string()
        };
        return (StatusCode::UNAUTHORIZED, Json(resp));
    };
    let Some(user_id) = db::user::fetches::getsession(&state.db, token.to_string()).await else {
        let resp = CreateRoomResponse {
            message: "Invalid token".to_string()
        };
        return (StatusCode::BAD_REQUEST, Json(resp));
    };
    // if public
    if payload.pop {
        match db::room::createroomdb::createroomdb(&state.db, user_id, payload.name, payload.all_players, payload.start_cash, payload.years, payload.pop, None).await {
            Ok(()) => {
                let resp = CreateRoomResponse {
                    message: "Room created".to_string()
                };
                (StatusCode::CREATED, Json(resp))
            }
            Err(e) => {
                println!("Error while creating a room in db: {}", e);
                let resp = CreateRoomResponse {
                    message: "Internal server error".to_string()
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(resp))
            }
        }
    } else {
        match db::room::createroomdb::createroomdb(&state.db, user_id, payload.name, payload.all_players, payload.start_cash, payload.years, payload.pop, Some(payload.password)).await {
            Ok(()) => {
                let resp = CreateRoomResponse {
                    message: "Room created".to_string()
                };
                (StatusCode::CREATED, Json(resp))
            }
            Err(e) => {
                println!("Error while creating a room in db: {}", e);
                let resp = CreateRoomResponse {
                    message: "Internal server error".to_string()
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(resp))
            }
        }
    }
}
