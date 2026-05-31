use sqlx::PgPool;
use uuid::Uuid;

use crate::handlers::queries::queryroom::Room;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct RoomRow {
    pub max_players: i32,
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
            max_players,
            start_money,
            duration_years
        FROM rooms
        "#
    )
    .bind(user_id)
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| Room {
            amount_players: row.max_players as u32,
            start_cash: row.start_money,
            years: row.duration_years as u32,
        })
        .collect())
}
