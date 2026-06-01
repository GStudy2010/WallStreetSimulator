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
            name TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL,
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
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS rooms (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

        owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

        name TEXT NOT NULL,
        max_players INTEGER NOT NULL CHECK (max_players > 0),
        current_players INTEGER NOT NULL,
        start_money DOUBLE PRECISION NOT NULL CHECK (start_money >= 0),
        duration_years INTEGER NOT NULL CHECK (duration_years > 0),
        public_private BOOLEAN NOT NULL,
        password TEXT,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "
        )
        .execute(db)
        .await
        .expect("Failed to create table rooms");
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS room_members (
        room_id UUID NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
        user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

        joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

        PRIMARY KEY (room_id, user_id)
        )    
        "
        )
        .execute(db)
        .await
        .expect("Failed to create table rooms");
    println!("Database setup complete");
}
