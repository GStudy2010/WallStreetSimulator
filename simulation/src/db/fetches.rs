use sqlx::{PgPool, prelude::FromRow};
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
#[derive(FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub email_verified: bool
}

pub async fn fetchuserbyemail(db: &PgPool, email: String) -> Option<User> {
    let result = sqlx::query_as::<_, User>(
        "
        SELECT id, name, email, password, email_verified
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
pub async fn getsession(db: &PgPool, token: String) -> Option<Uuid> {
    let result = sqlx::query_scalar::<_, Uuid>(
        "
        SELECT user_id
        FROM sessions
        WHERE token = $1
        AND expires_at > NOW()
        "
    )
    .bind(token)
    .fetch_one(db)
    .await;

    match result {
        Ok(user_id) => Some(user_id),
        Err(e) => {
            println!("Database error: {}", e);
            None
        }
    }
}
