use bigdecimal::BigDecimal;
use sqlx::{PgPool};

use crate::{db::error::DbError, entities::{Order, Product}};


pub async fn create(pool: &PgPool, order: Order) -> Result<i64, DbError> {
    let mut tx = pool.begin()
        .await
        .map_err(|e| match e {
            default => DbError::Internal(default)
        })?;
    let mut total_price = BigDecimal::from(0);
    for it in order.items.iter() {
        let p = sqlx::query_as!(
            Product,
            r#"SELECT * FROM products WHERE id = $1 FOR UPDATE"#,
            it.product_id,
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => DbError::NotFound,
            default => DbError::Internal(default)
        })?;
        total_price = total_price + p.price;
        if p.stock_quantity < it.quantity {
            return Err(DbError::Conflict("not enough supply".to_string()));
        }
    }

    let id: i64 = sqlx::query_scalar!(
        r#"INSERT INTO orders(user_id, total_price) VALUES($1, $2) RETURNING id"#,
        order.user_id,
        total_price
    )
    .fetch_one(&mut *tx).await
    .map_err(|e| match e {
        default => DbError::Internal(default)
    })?;


    for it in order.items.iter() {
        sqlx::query!(
            r#"UPDATE products SET stock_quantity = stock_quantity - $1 WHERE id = $2"#,
            it.quantity,
            it.product_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| match e {
            default => DbError::Internal(default)
        })?;
    }

    tx.commit()
        .await
        .map_err(|e| match e {
            default => DbError::Internal(default)
        })?;

    Ok(id)
}
