use axum::{extract::{Path, State}, http::StatusCode};

use crate::db::init::AppState;

pub async fn verifyemail(
    State(state): State<AppState>,
    Path(token): Path<String>,
    ) -> StatusCode {
    println!("Verify email link called for token: {}", token);
    let result = sqlx::query(
        "
        UPDATE users
        SET email_verified = true
        WHERE id = (
            SELECT user_id
            FROM email_verification
            WHERE token = $1
        )
        "
    )
    .bind(token)
    .execute(&state.db)
    .await;
    match result {
        Ok(res) if res.rows_affected() > 0 => StatusCode::OK,
        _ => StatusCode::BAD_REQUEST,
    }
}
