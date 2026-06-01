use sqlx::PgPool;
use uuid::Uuid;

use crate::handlers::queries::{queryroom::Room, queryuserinroom::User};
use sqlx::FromRow;

#[derive(FromRow)]
pub struct RoomRow {
    pub id: Uuid,
    pub name: String,
    pub max_players: i32,
    pub current_players: i32,
    pub start_money: f64,
    pub duration_years: i32,
}

pub async fn fetch_rooms(
    db: &PgPool,
    user_id: Uuid,
) -> Result<Vec<Room>, sqlx::Error> {
    let rows: Vec<RoomRow> = sqlx::query_as(
        r#"
        SELECT
            id,
            name,
            current_players,
            max_players,
            start_money,
            duration_years
        FROM rooms
        WHERE public_private = true
        "#
    )
    .bind(user_id)
    .fetch_all(db)
    .await?;
    Ok(rows
        .into_iter()
        .map(|row| Room {
            id: row.id,
            name: row.name,
            amount_players: row.max_players as u32,
            current_players: row.current_players as u32,
            start_cash: row.start_money,
            years: row.duration_years as u32,
        })
        .collect())
}
#[derive(FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub name: String,
    pub typeof_user: bool,
}

pub async fn fetch_users(
    db: &PgPool,
    room_id: Uuid,
) -> Result<Vec<User>, sqlx::Error> {
    let rows: Vec<UserRow> = sqlx::query_as(
        r#"
        SELECT
            u.id,
            u.name,
            rm.typeof_user
        FROM room_members rm
        INNER JOIN users u
            ON u.id = rm.user_id
        WHERE rm.room_id = $1
        ORDER BY rm.joined_at
        "#
    )
    .bind(room_id)
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| User {
            id: row.id,
            name: row.name,
            typeofuser: row.typeof_user,
        })
        .collect())
}
