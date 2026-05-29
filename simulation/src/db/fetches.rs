use sqlx::PgPool;
use uuid::Uuid;

pub async fn fetchidbyemail(db: &PgPool, email: String) -> Option<Uuid> {
    let result = sqlx::query_scalar::<_, Uuid>(
        "
        SELECT id
        FROM users
        WHERE email = $1
        "
    )
        .bind(email)
        .fetch_one(db)
        .await;
    match result {
        Ok(id) => Some(id),
        Err(e) => {
            println!("Database error: {}", e);
            None
        }
    }
}
