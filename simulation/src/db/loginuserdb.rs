use sqlx::PgPool;
use uuid::Uuid;

pub async fn createsession(db: &PgPool, user_id: Uuid) -> Option<String> {
    let token = Uuid::new_v4().to_string();
    let result = sqlx::query(
        "
        INSERT INTO sessions (token, user_id, expires_at)
        VALUES ($1, $2, NOW() + INTERVAL '7 days')
        "
    )
        .bind(token.clone())
        .bind(user_id)
        .execute(db)
        .await;
    match result {
        Ok(_) => Some(token),
        Err(e) => {
            println!("Database error: {}", e);
            None
        }
    }
}
