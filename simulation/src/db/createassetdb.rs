use sqlx::PgPool;
use uuid::Uuid;

pub async fn assetexists(db: &PgPool, symbol: &str) -> Result<bool, sqlx::Error> {
    let result: bool = sqlx::query_scalar(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM assets_on_market
            WHERE symbol = $1
        )
        "#
    )
    .bind(symbol)
    .fetch_one(db)
    .await?;
    Ok(result)
}
pub async fn createasset(
    db: &PgPool,
    symbol: &str,
    quantity: i32,
    price: f32,
    portfolio_id: Uuid,
) -> Result<(), sqlx::Error> {
    let mut tx = db.begin().await?;

    let money: f64 = sqlx::query_scalar(
        "
        SELECT money
        FROM portfolios
        WHERE id = $1
        "
    )
    .bind(portfolio_id)
    .fetch_one(&mut *tx)
    .await?;

    let cost = quantity as f64 * price as f64;

    if money < cost {
        tx.rollback().await?;
        return Err(sqlx::Error::RowNotFound);
    }

    sqlx::query(
        "
        UPDATE portfolios
        SET money = money - $1
        WHERE id = $2
        "
    )
    .bind(cost)
    .bind(portfolio_id)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "
        INSERT INTO assets_on_market
            (portfolio_id, symbol, quantity, max_price)
        VALUES
            ($1, $2, $3, $4)
        "
    )
    .bind(portfolio_id)
    .bind(symbol)
    .bind(quantity as f64)
    .bind(price as f64)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "
        INSERT INTO assets_on_market_user
            (portfolio_id, symbol, quantity, price)
        VALUES
            ($1, $2, $3, $4)
        "
    )
    .bind(portfolio_id)
    .bind(symbol)
    .bind(quantity as f64)
    .bind(price as f64)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(())
}
