use std::net::SocketAddr;
use axum::routing::post;
use serde::{Deserialize, Serialize};
use axum::{Json, Router, http::StatusCode, response::IntoResponse};
use tower_http::cors::CorsLayer;
use sqlx::{PgPool, postgres::PgPoolOptions};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

#[derive(Debug, Deserialize)]
struct TestRouteHandlerRequest {
    test: String,
}
#[derive(Debug, Serialize)]
struct TestRouteHandlerResponse{
    test: String,
}

async fn test_route_handler(
    Json(payload): Json<TestRouteHandlerRequest>,
    ) -> impl IntoResponse {
    println!("Test route has been called, the message: {:?}", payload.test);
    let resp = TestRouteHandlerResponse {
        test: "Test".to_string(),
    };
    (StatusCode::OK, Json(resp))
}

async fn connect_db(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .expect("Failed to connect")
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = connect_db(&database_url).await;
    let _state = AppState {
        db: pool,
    };
    let server = Router::new()
        .route("/api/test", post(test_route_handler))
        .layer(CorsLayer::permissive());
    let addr = SocketAddr::from(([127, 0, 0, 1], 42069));

    println!("Server started");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();
    axum::serve(listener, server).await.unwrap();
    tokio::signal::ctrl_c().await.unwrap();
}
