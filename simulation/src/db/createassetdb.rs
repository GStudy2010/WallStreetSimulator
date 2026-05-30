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
pub async fn createasset(db: &PgPool, symbol: &str, quantity: i32, price: f32, portfolio_id: Uuid) -> Result<(), sqlx::Error> {
    let _result = sqlx::query(
        "
        INSERT INTO assets_on_market (portfolio_id, symbol, quantity, max_price)
        VALUES($1, $2, $3, $4)
        "
    )
    .bind(portfolio_id)
    .bind(symbol)
    .bind(quantity)
    .bind(price)
    .execute(db)
    .await?;
    let _result2 = sqlx::query(
        "
        INSERT INTO assets_on_market_user (portfolio_id, symbol, quantity, price)
        VALUES($1, $2, $3, $4)
        "
    )
    .bind(portfolio_id)
    .bind(symbol)
    .bind(quantity)
    .bind(price)
    .execute(db)
    .await?;
    Ok(())
}
