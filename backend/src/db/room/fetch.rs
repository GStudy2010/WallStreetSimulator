use sqlx::PgPool;
use uuid::Uuid;

pub async fn fetchpasswordbyname(
    db: &PgPool,
    name: String,
) -> Result<String, sqlx::Error> {
    let password_hash: String = sqlx::query_scalar(
        r#"
        SELECT password_hash
        FROM rooms
        WHERE name = $1
        "#
    )
    .bind(name)
    .fetch_one(db)
    .await?;

    Ok(password_hash)
}
pub async fn fetchroomidbyname(
    db: &PgPool,
    name: String,
) -> Result<Uuid, sqlx::Error> {
    let password_hash: Uuid = sqlx::query_scalar(
        r#"
        SELECT id
        FROM rooms
        WHERE name = $1
        "#
    )
    .bind(name)
    .fetch_one(db)
    .await?;

    Ok(password_hash)
}
