use sqlx::{PgPool, Row};
use uuid::Uuid;

pub async fn joinroom(
    db: &PgPool,
    room_id: Uuid,
    user_id: Uuid,
) -> Result<Option<String>, sqlx::Error> {
    let room = sqlx::query(
        r#"
        SELECT owner_id, current_players, max_players
        FROM rooms
        WHERE id = $1
        "#
    )
    .bind(room_id)
    .fetch_one(db)
    .await?;

    let owner_id: Uuid = room.get("owner_id");
    let current_players: i32 = room.get("current_players");
    let max_players: i32 = room.get("max_players");

    let is_owner = owner_id == user_id;

    if !is_owner && current_players >= max_players {
        return Ok(None);
    }

    let token = Uuid::new_v4().to_string();

    let mut tx = db.begin().await?;

    sqlx::query(
        r#"
        INSERT INTO room_members
        (
            room_id,
            user_id,
            typeof_user,
            token,
            expires_at
        )
        VALUES
        (
            $1,
            $2,
            $3,
            $4,
            NOW() + INTERVAL '7 days'
        )
        "#
    )
    .bind(room_id)
    .bind(user_id)
    .bind(!is_owner) // false = admin, true = normal user
    .bind(&token)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        UPDATE rooms
        SET current_players = current_players + 1
        WHERE id = $1
        "#
    )
    .bind(room_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Some(token))
}
