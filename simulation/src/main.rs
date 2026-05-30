use std::net::SocketAddr;
use axum::routing::{post, get};
use axum::Router;
use tower_http::cors::CorsLayer;
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
    let pool = db::init::connect_db(&database_url).await;
    let state = db::init::AppState {
        db: pool,
    };
    db::init::setup_database(&state.db).await;
    let server = Router::new()
        .route("/api/test", post(handlers::apitest::test_route_handler))
        .route("/api/createuser", post(handlers::createuser::create_user_handler))
        .route("/api/loginuser", post(handlers::loginuser::login_user_handler))
        .route("/api/logoutuser", post(handlers::logoutuser::logout_user_handler))
        .route("/api/verifyemail/{token}", get(handlers::verifyemail::verifyemail))
        .route("/api/createasset", post(handlers::createasset::create_asset_handler))
        .route("/api/sellasset", post(handlers::sellasset::sell_asset_handler))
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
