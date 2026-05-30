use sqlx::PgPool;
use uuid::Uuid;
pub async fn buyassets(
    db: &PgPool,
    symbol: &str,
    quantity: i32,
    max_price: f64,
    portfolio_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let mut tx = db.begin().await?;

    let stock: Option<(f64, f64, Uuid)> = sqlx::query_as(
        r#"
        SELECT quantity, price, portfolio_id
        FROM assets_on_market_free
        WHERE symbol = $1
        LIMIT 1
        "#
    )
    .bind(symbol)
    .fetch_optional(&mut *tx)
    .await?;

    let Some((available_quantity, market_price, seller_id)) = stock else {
        tx.rollback().await?;
        return Ok(false);
    };

    if available_quantity < quantity as f64 {
        tx.rollback().await?;
        return Ok(false);
    }

    if market_price > max_price {
        tx.rollback().await?;
        return Ok(false);
    }

    let total_cost = market_price * quantity as f64;

    let money: f64 = sqlx::query_scalar(
        r#"
        SELECT money
        FROM portfolios
        WHERE id = $1
        "#
    )
    .bind(portfolio_id)
    .fetch_one(&mut *tx)
    .await?;

    if money < total_cost {
        tx.rollback().await?;
        return Ok(false);
    }

    sqlx::query(
        r#"
        UPDATE portfolios
        SET money = money - $1
        WHERE id = $2
        "#
    )
    .bind(total_cost)
    .bind(portfolio_id)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        UPDATE portfolios
        SET money = money + $1
        WHERE id = $2
        "#
    )
    .bind(total_cost)
    .bind(seller_id)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        UPDATE assets_on_market_free
        SET quantity = quantity - $1
        WHERE symbol = $2
        "#
    )
    .bind(quantity as f64)
    .bind(symbol)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        DELETE FROM assets_on_market_free
        WHERE symbol = $1
          AND quantity <= 0
        "#
    )
    .bind(symbol)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO assets_on_market_user
            (portfolio_id, symbol, quantity, price)
        VALUES
            ($1, $2, $3, $4)
        "#
    )
    .bind(portfolio_id)
    .bind(symbol)
    .bind(quantity as f64)
    .bind(market_price)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(true)
}
