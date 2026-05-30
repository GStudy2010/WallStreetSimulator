use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{db::{self, init::AppState}, helpers};
#[derive(Deserialize)]
pub struct CreateUserHandlerRequest {
    name: String,
    email: String,
    password: String,
}
#[derive(Serialize)]
pub struct CreateUserHandlerResponse {
    message: String,
}


pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserHandlerRequest>,
) -> impl IntoResponse {
    println!("Create user handler called");
    if !helpers::check_email(payload.email.clone()).await {
        let resp = CreateUserHandlerResponse {
            message: "Email is incorrect".to_string(),
        };
        return (StatusCode::BAD_REQUEST, Json(resp));
    }
    let npass  = helpers::hash(payload.password).await;
    if !db::createuserdb::saveuser(&state.db, payload.name, payload.email.clone(), npass).await {
        let resp = CreateUserHandlerResponse {
            message: "Error while saving to database".to_string(),
        };
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(resp));
    }
    let resp = CreateUserHandlerResponse {
        message: "User created, email to verify account sent".to_string(),
    };
    let Some(user_id) = db::fetches::fetchidbyemail(&state.db, payload.email.clone()).await else {
        let resp = CreateUserHandlerResponse {
            message: "Error while fetching user".to_string(),
        };
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(resp));
    };
    let token = Uuid::new_v4().to_string();
    if !db::createuserdb::saveemailverification(&state.db, token.clone(), user_id).await {
        let resp = CreateUserHandlerResponse {
            message: "Error while saving to database".to_string(),
        };
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(resp));
    }
    let link = "https://surfing-robinson-canvas-athletes.trycloudflare.com/api/verifyemail/".to_string() + &token;
    if !helpers::send_email(payload.email.clone(), link).await {
        let resp = CreateUserHandlerResponse {
            message: "Error while emailing you verification".to_string(),
        };
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(resp));
    }
    if !db::createuserdb::saveportfolio(&state.db, user_id).await {
        let resp = CreateUserHandlerResponse {
            message: "Error while database insertion".to_string(),
        };
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(resp));
    }
    (StatusCode::CREATED, Json(resp))
}
