use std::net::SocketAddr;
use axum::http::Request;
use axum::middleware::{self, Next};
use axum::response::{Redirect, Response};
use axum::routing::{post, get};
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};
mod db;
mod handlers;
mod helpers;


#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    println!("SMTP_EMAIL: {:?}", std::env::var("SMTP_EMAIL"));
    println!("SMTP_PASSWORD: {:?}", std::env::var("SMTP_PASSWORD"));
    println!("DATABASE_URL: {:?}", std::env::var("DATABASE_URL"));
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = db::user::init::connect_db(&database_url).await;
    let state = db::user::init::AppState {
        db: pool,
    };
    db::user::init::setup_database(&state.db).await;
    let react_service = ServeDir::new("../frontend/dist")
        .not_found_service(
            ServeFile::new("../frontend/dist/index.html")
            );
    let server = Router::new()
        .nest_service("/app", react_service)
        .route("/api/test", post(handlers::user::apitest::test_route_handler))
        .route("/api/createuser", post(handlers::user::createuser::create_user_handler))
        .route("/api/loginuser", post(handlers::user::loginuser::login_user_handler))
        .route("/api/logoutuser", post(handlers::user::logoutuser::logout_user_handler))
        .route("/api/verifyemail/{token}", get(handlers::user::verifyemail::verifyemail))
        .route("/api/createroom", post(handlers::room::createroom::create_room_handler))
        .route("/api/joinroom", post(handlers::room::joinroom::join_room_handler))
        .route("/api/query/rooms", get(handlers::queries::queryroom::query_room_handler))
        .with_state(state)
        .layer(CorsLayer::permissive());
    let addr = SocketAddr::from(([127, 0, 0, 1], 42069));

    println!("Server started");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();
    axum::serve(listener, server).await.unwrap();
    tokio::signal::ctrl_c().await.unwrap();
}
