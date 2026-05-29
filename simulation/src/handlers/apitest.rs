use serde::{Deserialize, Serialize};
use axum::{Json, http::StatusCode, response::IntoResponse};
#[derive(Debug, Deserialize)]
pub struct TestRouteHandlerRequest {
    test: String,
}
#[derive(Debug, Serialize)]
pub struct TestRouteHandlerResponse{
    test: String,
}

pub async fn test_route_handler(
    Json(payload): Json<TestRouteHandlerRequest>,
    ) -> impl IntoResponse {
    println!("Test route has been called, the message: {:?}", payload.test);
    let resp = TestRouteHandlerResponse {
        test: "Test".to_string(),
    };
    (StatusCode::OK, Json(resp))
}
