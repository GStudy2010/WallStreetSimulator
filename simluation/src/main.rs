use std::net::SocketAddr;
use axum::routing::post;
use serde::{Deserialize, Serialize};
use axum::{Json, Router, http::StatusCode, response::IntoResponse};
use tower_http::cors::CorsLayer;

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

#[tokio::main]
async fn main() {
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
