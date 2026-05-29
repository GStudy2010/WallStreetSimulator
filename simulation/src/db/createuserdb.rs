use sqlx::PgPool;
use uuid::Uuid;


// User id missing
pub async fn saveemailverification(db: &PgPool, token: String, user_id: Uuid) -> bool {
    let result = sqlx::query(
        "
        INSERT INTO email_verification (token, user_id, expires_at)
        VALUES($1, $2, NOW() + INTERVAL '24 hours')"
    )
    .bind(token)
    .bind(user_id)
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
pub async fn saveuser(db: &PgPool, name: String, email: String, pass: String) -> bool {
    let result = sqlx::query(
        "
        INSERT INTO users (name, email, password, email_verified)
        VALUES ($1, $2, $3, false)
        "
    )
    .bind(name)
    .bind(email)
    .bind(pass)
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
