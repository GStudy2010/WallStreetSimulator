use sqlx::PgPool;
use uuid::Uuid;

use crate::handlers::queries::queryroom::Room;
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
