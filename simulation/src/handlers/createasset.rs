use axum::{Json, extract::State, http::{HeaderMap, StatusCode}, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::db::{self, init::AppState};

#[derive(Deserialize)]
pub struct CreateAssetHandlerRequest {
    symbol: String,
    quantity: i32,
    price: f32
}

#[derive(Serialize)]
pub struct CreateAssetHandlerResponse {
    message: String,
}
pub async fn create_asset_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateAssetHandlerRequest>,
) -> impl IntoResponse {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));
    let Some(token) = token else {
        let resp = CreateAssetHandlerResponse {
            message: "No token provided".to_string()
        };
        return (StatusCode::UNAUTHORIZED, Json(resp));
    };
    let Some(user_id) = db::fetches::getsession(&state.db, token.to_string()).await else {
        let resp = CreateAssetHandlerResponse {
            message: "No user with that token".to_string()
        };
        return (StatusCode::UNAUTHORIZED, Json(resp));
    };
    match db::createassetdb::assetexists(&state.db, &payload.symbol).await {
        Ok(true) => {
            let resp = CreateAssetHandlerResponse {
                message: "Stock with that name already exists".to_string(),
            };

            (StatusCode::BAD_REQUEST, Json(resp))
        }

        Ok(false) => {
            let Some(portfolio_id) = db::fetches::fetchportfolioidbyid(&state.db, user_id).await else {
                let resp = CreateAssetHandlerResponse {
                    message: "You don't have a portfolio".to_string(),
                };
                return (StatusCode::BAD_REQUEST, Json(resp));
            };
            let Some(portfolio) = db::fetches::fetchportfoliobyid(&state.db, portfolio_id).await else {
                let resp = CreateAssetHandlerResponse {
                    message: "You don't have a portfolio".to_string(),
                };
                return (StatusCode::BAD_REQUEST, Json(resp));
            };
            if portfolio.money < (payload.price as f64) * (payload.quantity as f64) {
                let resp = CreateAssetHandlerResponse {
                    message: "You don't have enough money to create this stock".to_string(),
                };
                return (StatusCode::BAD_REQUEST, Json(resp));
            }
            db::createassetdb::createasset(
                &state.db,
                &payload.symbol,
                payload.quantity,
                payload.price,
                portfolio_id 
            )
            .await
            .unwrap();

            let resp = CreateAssetHandlerResponse {
                message: "Created an asset".to_string(),
            };

            (StatusCode::OK, Json(resp))
        }

        Err(_) => {
            let resp = CreateAssetHandlerResponse {
                message: "Database error".to_string(),
            };

            (StatusCode::INTERNAL_SERVER_ERROR, Json(resp))
        }
    }
}
