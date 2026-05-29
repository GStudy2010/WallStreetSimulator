use sqlx::{PgPool, postgres::PgPoolOptions};
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub async fn connect_db(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .expect("Failed to connect")
}

pub async fn setup_database(db: &PgPool) {

    sqlx::query(
        r#"
        CREATE EXTENSION IF NOT EXISTS "pgcrypto";
        "#
    )
    .execute(db)
    .await
    .expect("Failed to enable pgcrypto");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            email_verified BOOLEAN NOT NULL DEFAULT FALSE,
            created_at TIMESTAMP NOT NULL DEFAULT NOW()
        );
        "#
    )
    .execute(db)
    .await
    .expect("Failed to create users table");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS email_verification (
            token TEXT PRIMARY KEY,
            user_id UUID NOT NULL REFERENCES users(id),
            expires_at TIMESTAMP NOT NULL
        );
        "#
    )
    .execute(db)
    .await
    .expect("Failed to create users table");
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS sessions (
            token TEXT PRIMARY KEY,
            user_id UUID NOT NULL REFERENCES users(id),
            expires_at TIMESTAMP NOT NULL
        )
        "
        )
        .execute(db)
        .await
        .expect("Falied to create sessions table");

    println!("Database setup complete");
}
