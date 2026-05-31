use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{db::{self, user::init::AppState}, helpers};

#[derive(Deserialize)]
pub struct LoginUserRequest {
    email: String,
    password: String,
}
#[derive(Serialize)]
pub struct LoginUserResponse {
    message: String,
}
pub async fn login_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginUserRequest>
    ) -> impl IntoResponse {
    if !helpers::check_email(payload.email.clone()).await {
        let resp = LoginUserResponse {
            message: "Not a valid email".to_string(),
        };
        return (StatusCode::BAD_REQUEST, Json(resp));
    }
    let Some(user) = db::user::fetches::fetchuserbyemail(&state.db, payload.email).await else {
        let resp = LoginUserResponse {
            message: "Invalid email of password".to_string(),
        };
        return (StatusCode::BAD_REQUEST, Json(resp));
    };
    let parsed_hash = PasswordHash::new(&user.password).unwrap();
    if Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash).is_err() {
        let resp = LoginUserResponse {
            message: "Incorrect password".to_string(),
        };
        return (StatusCode::BAD_REQUEST, Json(resp));
    }
    if !user.email_verified {
        let resp = LoginUserResponse {
            message: "Please verify your email first".to_string(),
        };
        return (StatusCode::FORBIDDEN, Json(resp));
    }
    let Some(token) = db::user::loginuserdb::createsession(&state.db, user.id).await else {
        let resp = LoginUserResponse {
            message: "Internal server error".to_string(),
        };
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(resp));
    };
    let resp = LoginUserResponse {
        message: token,
    };
    (StatusCode::OK, Json(resp))

}
