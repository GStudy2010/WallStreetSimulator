use sqlx::PgPool;

pub async fn deletesession(db: &PgPool, token: String) -> bool {
    let result = sqlx::query(
        "DELETE FROM sessions WHERE token = $1"
    )
    .bind(token)
    .execute(db)
    .await;

    match result {
        Ok(_) => true,
        Err(e) => {
            println!("Database error: {}", e);
            false
        }
    }
}
