use sqlx::PgPool;
use uuid::Uuid;

pub async fn start_game(
    db: &PgPool,
    room_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        UPDATE rooms
        SET started = true
        WHERE id = $1
        "
    )
    .bind(room_id)
    .execute(db)
    .await?;

    Ok(())
}
