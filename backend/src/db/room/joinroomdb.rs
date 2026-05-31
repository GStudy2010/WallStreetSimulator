use sqlx::PgPool;
use uuid::Uuid;

pub async fn joinroomdb(
    db: &PgPool,
    room_id: Uuid,
    user_id: Uuid,
) -> Result<(), sqlx::Error> {
    let current_players: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM room_members
        WHERE room_id = $1
        "#
    )
    .bind(room_id)
    .fetch_one(db)
    .await?;

    let max_players: i32 = sqlx::query_scalar(
        r#"
        SELECT max_players
        FROM rooms
        WHERE id = $1
        "#
    )
    .bind(room_id)
    .fetch_one(db)
    .await?;

    if current_players >= max_players as i64 {
        return Err(sqlx::Error::RowNotFound);
    }
    sqlx::query(
        r#"
        INSERT INTO room_members (room_id, user_id)
        VALUES ($1, $2)
        ON CONFLICT (room_id, user_id) DO NOTHING
        "#
    )
    .bind(room_id)
    .bind(user_id)
    .execute(db)
    .await?;
    Ok(())
}
