use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::db::{self, init::AppState};

#[derive(Deserialize)]
pub struct BuyAssetHandlerRequest {
    symbol: String,
    amount: i32,
    max_price: f64,
}

#[derive(Serialize)]
pub struct BuyAssetHandlerResponse {
    message: String,
}

pub async fn buy_asset_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<BuyAssetHandlerRequest>,
) -> impl IntoResponse {
    if payload.amount <= 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(BuyAssetHandlerResponse {
                message: "Amount must be greater than 0".to_string(),
            }),
        );
    }

    if payload.max_price <= 0.0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(BuyAssetHandlerResponse {
                message: "Price must be greater than 0".to_string(),
            }),
        );
    }

    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    let Some(token) = token else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(BuyAssetHandlerResponse {
                message: "No token provided".to_string(),
            }),
        );
    };

    let Some(user_id) = db::fetches::getsession(&state.db, token.to_string()).await else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(BuyAssetHandlerResponse {
                message: "Invalid session token".to_string(),
            }),
        );
    };

    let Some(portfolio_id) =
        db::fetches::fetchportfolioidbyid(&state.db, user_id).await
    else {
        return (
            StatusCode::BAD_REQUEST,
            Json(BuyAssetHandlerResponse {
                message: "You don't have a portfolio".to_string(),
            }),
        );
    };

    match db::buyassetdb::buyassets(
        &state.db,
        &payload.symbol,
        payload.amount,
        payload.max_price,
        portfolio_id,
    )
    .await
    {
        Ok(true) => (
            StatusCode::OK,
            Json(BuyAssetHandlerResponse {
                message: "Stock bought for sale".to_string(),
            }),
        ),

        Ok(false) => (
            StatusCode::BAD_REQUEST,
            Json(BuyAssetHandlerResponse {
                message: "Not enough shares to buy".to_string(),
            }),
        ),

        Err(e) => {
            eprintln!("Database error: {}", e);

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(BuyAssetHandlerResponse {
                    message: "Internal server error".to_string(),
                }),
            )
        }
    }
}
