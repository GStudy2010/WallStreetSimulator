use std::env;

use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use lettre::{Message, SmtpTransport, Transport, message::Mailbox, transport::smtp::authentication::Credentials};
use rand::rngs::OsRng;
use regex::Regex;

pub async fn send_email(recipient: String, verify_link: String) -> bool {
    let smtp_email = env::var("SMTP_EMAIL").unwrap();
    let smtp_password = env::var("SMTP_PASSWORD").unwrap();

    let email = Message::builder()
        .from("wallstreetsim <wallstreet.sender@gmail.com>".parse::<Mailbox>().unwrap())
        .to(recipient.parse().unwrap())
        .subject("Account verification")
        .body(format!("Click this link to verify your account:\n\n{}", verify_link))
        .unwrap();

    let creds = Credentials::new(smtp_email, smtp_password);

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match tokio::task::spawn_blocking(move || mailer.send(&email)).await {
        Ok(Ok(_)) => true,
        Ok(Err(e)) => {
            println!("SMTP error: {}", e);
            false
        }
        Err(e) => {
            println!("Thread panic: {}", e);
            false
        }
}}
pub async fn check_email(email: String) -> bool {
    let re = Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    ).unwrap();
    re.is_match(email.as_str())
}

pub async fn hash(string: String) -> String {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    argon2
        .hash_password(string.as_bytes(), &salt)
        .unwrap()
        .to_string()
}
use serde::Serialize;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}
#[derive(Debug)]
pub enum ApiError {
    Unauthorized,
    InvalidToken,
    Database,
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::Unauthorized =>
                (StatusCode::UNAUTHORIZED, "No token provided"),

            ApiError::InvalidToken =>
                (StatusCode::BAD_REQUEST, "Invalid token"),

            ApiError::Database =>
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
        };

        (
            status,
            Json(ErrorResponse {
                message: message.to_string(),
            }),
        )
        .into_response()
    }
}
