use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::db::{self, init::AppState};

#[derive(Deserialize)]
pub struct SellAssetHandlerRequest {
    symbol: String,
    amount: i32,
    price: f64,
}

#[derive(Serialize)]
pub struct SellAssetHandlerResponse {
    message: String,
}

pub async fn sell_asset_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<SellAssetHandlerRequest>,
) -> impl IntoResponse {
    if payload.amount <= 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(SellAssetHandlerResponse {
                message: "Amount must be greater than 0".to_string(),
            }),
        );
    }

    if payload.price <= 0.0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(SellAssetHandlerResponse {
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
            Json(SellAssetHandlerResponse {
                message: "No token provided".to_string(),
            }),
        );
    };

    let Some(user_id) = db::fetches::getsession(&state.db, token.to_string()).await else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(SellAssetHandlerResponse {
                message: "Invalid session token".to_string(),
            }),
        );
    };

    let Some(portfolio_id) =
        db::fetches::fetchportfolioidbyid(&state.db, user_id).await
    else {
        return (
            StatusCode::BAD_REQUEST,
            Json(SellAssetHandlerResponse {
                message: "You don't have a portfolio".to_string(),
            }),
        );
    };

    match db::sellassetdb::sellassets(
        &state.db,
        &payload.symbol,
        payload.amount,
        payload.price,
        portfolio_id,
    )
    .await
    {
        Ok(true) => (
            StatusCode::OK,
            Json(SellAssetHandlerResponse {
                message: "Stock listed for sale".to_string(),
            }),
        ),

        Ok(false) => (
            StatusCode::BAD_REQUEST,
            Json(SellAssetHandlerResponse {
                message: "Not enough shares to sell".to_string(),
            }),
        ),

        Err(e) => {
            eprintln!("Database error: {}", e);

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SellAssetHandlerResponse {
                    message: "Internal server error".to_string(),
                }),
            )
        }
    }
}
