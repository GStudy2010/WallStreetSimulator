use sqlx::PgPool;
use uuid::Uuid;
pub async fn sellassets(
    db: &PgPool,
    symbol: &str,
    quantity: i32,
    price: f64,
    portfolio_id: Uuid,
) -> Result<bool, sqlx::Error> {

    let mut tx = db.begin().await?;

    let owned: Option<f64> = sqlx::query_scalar(
        r#"
        SELECT quantity
        FROM assets_on_market_user
        WHERE portfolio_id = $1
          AND symbol = $2
        "#
    )
    .bind(portfolio_id)
    .bind(symbol)
    .fetch_optional(&mut *tx)
    .await?;

    let Some(owned) = owned else {
        tx.rollback().await?;
        return Ok(false);
    };

    if owned < quantity as f64 {
        tx.rollback().await?;
        return Ok(false);
    }

    sqlx::query(
        r#"
        UPDATE assets_on_market_user
        SET quantity = quantity - $1
        WHERE portfolio_id = $2
          AND symbol = $3
        "#
    )
    .bind(quantity as f64)
    .bind(portfolio_id)
    .bind(symbol)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        DELETE FROM assets_on_market_user
        WHERE portfolio_id = $1
          AND symbol = $2
          AND quantity <= 0
        "#
    )
    .bind(portfolio_id)
    .bind(symbol)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO assets_on_market_free (symbol, quantity, price, portfolio_id)
        VALUES ($1, $2, $3, $4)
        "#
    )
    .bind(symbol)
    .bind(quantity as f64)
    .bind(price)
    .bind(portfolio_id)
    .execute(&mut *tx)
    .await?;

    let current_max: Option<f64> = sqlx::query_scalar(
        r#"
        SELECT max_price
        FROM assets_on_market
        WHERE portfolio_id = $1
          AND symbol = $2
        "#
    )
    .bind(portfolio_id)
    .bind(symbol)
    .fetch_optional(&mut *tx)
    .await?;

    match current_max {
        Some(max_price) => {
            if price > max_price {
                sqlx::query(
                    r#"
                    UPDATE assets_on_market
                    SET max_price = $1
                    WHERE portfolio_id = $2
                      AND symbol = $3
                    "#
                )
                .bind(price)
                .bind(portfolio_id)
                .bind(symbol)
                .execute(&mut *tx)
                .await?;
            }
        }
        None => {
            sqlx::query(
                r#"
                INSERT INTO assets_on_market
                    (portfolio_id, symbol, quantity, max_price)
                VALUES
                    ($1, $2, $3, $4)
                "#
            )
            .bind(portfolio_id)
            .bind(symbol)
            .bind(quantity as f64)
            .bind(price)
            .execute(&mut *tx)
            .await?;
        }
    }

    tx.commit().await?;

    Ok(true)
}
pub async fn enoughstock(
    db: &PgPool,
    symbol: &str,
    quantity: i32,
    portfolio_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let owned: Option<i32> = sqlx::query_scalar(
        r#"
        SELECT quantity
        FROM assets_on_market_user
        WHERE portfolio_id = $1
          AND symbol = $2
        "#
    )
    .bind(portfolio_id)
    .bind(symbol)
    .fetch_optional(db)
    .await?;

    match owned {
        Some(current_quantity) => Ok(current_quantity >= quantity),
        None => Ok(false),
    }
}
