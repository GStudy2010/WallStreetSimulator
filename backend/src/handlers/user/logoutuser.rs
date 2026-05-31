use axum::{Json, extract::State, http::{HeaderMap, StatusCode}, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::db::{self, user::init::AppState};

#[derive(Serialize, Deserialize)]
pub struct LogoutUserHandlerResponse {
    message: String,
}

pub async fn logout_user_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));
    let Some(token) = token else {
        let resp = LogoutUserHandlerResponse {
            message: "No token provided".to_string()
        };
        return (StatusCode::UNAUTHORIZED, Json(resp));
    };
    if !db::user::logoutuserdb::deletesession(&state.db, token.to_string()).await {
        let resp = LogoutUserHandlerResponse {
            message: "Error while logging out".to_string()
        };
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(resp));
    }

    let resp = LogoutUserHandlerResponse {
        message: "Logged out successfully".to_string() 
    };
    (StatusCode::OK, Json(resp))
}
