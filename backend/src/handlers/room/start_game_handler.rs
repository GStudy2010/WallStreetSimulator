use axum::{Json, extract::State, http::{HeaderMap, StatusCode}, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{db::{self, user::init::AppState}, helpers::ApiError};

#[derive(Serialize, Deserialize)]
pub struct SomeStruct {
    message: String,
}

pub async fn start_game(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    println!("STARTGAME");
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));
    let Some(token) = token else {
        let resp = SomeStruct {
            message: "No token provided".to_string()
        };
        return (StatusCode::UNAUTHORIZED, Json(resp));
    };
    let Some(user_id) = db::user::fetches::getsession(&state.db, token.to_string()).await else {
        let resp = SomeStruct {
            message: "Invalid token".to_string()
        };
        return (StatusCode::BAD_REQUEST, Json(resp));
    };
    let r_token = headers
        .get("Room-Token")
        .and_then(|v| v.to_str().ok());
    let Some(r_token) = r_token else {
        let resp = SomeStruct {
            message: "No token provided".to_string()
        };
        return (StatusCode::UNAUTHORIZED, Json(resp));
    };
    let Some(room_id) = db::user::fetches::getroom(&state.db, r_token.to_string()).await else {
        let resp = SomeStruct {
            message: "Invalid token".to_string()
        };
        return (StatusCode::BAD_REQUEST, Json(resp));
    };
    let resp = SomeStruct {
        message: "Start game".to_string(),
    };

    let message = serde_json::json!({
        "type": "GAME_STARTED"
    })
    .to_string();

    let mut rooms = state.rooms.write().await;

    if let Some(room) = rooms.get_mut(r_token) {
        room.game_started = true;

        room.clients.retain(|client| {
            client.send(message.clone()).is_ok()
        });
    }
    match db::room::startroomdb::start_game(&state.db, room_id).await {
        Ok(()) => {
            (StatusCode::OK, Json(resp))
        }
        Err(e) => {
            println!("error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(resp))
        }
    }
}
