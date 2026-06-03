use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use std::net::SocketAddr;
use axum::routing::{post, get};
use axum::Router;
use tokio::sync::{mpsc, RwLock};
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
        rooms: Arc::new(RwLock::new(HashMap::new())),
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
        .route("/api/query/idbyauthtoken", get(handlers::queries::queryidbyauthtoken::queryidbyauthtoken))
        .route("/api/query/rooms", get(handlers::queries::queryroom::query_room_handler))
        .route("/api/query/usersinroom", get(handlers::queries::queryuserinroom::query_user_in_room_handler))
        .route("/api/startgame", post(handlers::room::start_game_handler::start_game))
        .route("/ws", get(handlers::room::ws_handler::ws_handler))
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
