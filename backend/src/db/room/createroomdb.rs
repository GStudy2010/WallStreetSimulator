use sqlx::PgPool;
use uuid::Uuid;


pub async fn createroomdb(
    db: &PgPool,
    user_id: Uuid,
    rname: String,
    rplayers: u32,
    rmoney: f64,
    rtime: u32,
    public_private: bool,
    password: Option<String>
) -> Result<(), sqlx::Error> {
    if public_private {
        sqlx::query(
            r#"
        INSERT INTO rooms (
            owner_id,
            name,
            max_players,
            current_players,
            start_money,
            duration_years,
            public_private,
            started
        )
        VALUES ($1, $2, $3, 0, $4, $5, $6, false)
        "#
        )
            .bind(user_id)
            .bind(rname)
            .bind(rplayers as i32)
            .bind(rmoney)
            .bind(rtime as i32)
            .bind(public_private)
            .execute(db)
            .await?;
    } else {
        let mut p: String = String::new();
        match password {
            Some(s) => p = s,
            None => println!("   a"),
        }
        sqlx::query(
            r#"
        INSERT INTO rooms (
            owner_id,
            name,
            max_players,
            current_players,
            start_money,
            duration_years,
            public_private,
            password,
            started
        )
        VALUES ($1, $2, $3, 0, $4, $5, $6, $7, false)
        "#
        )
            .bind(user_id)
            .bind(rname)
            .bind(rplayers as i32)
            .bind(rmoney)
            .bind(rtime as i32)
            .bind(public_private)
            .bind(p)
            .execute(db)
            .await?;
    }

    Ok(())
}
