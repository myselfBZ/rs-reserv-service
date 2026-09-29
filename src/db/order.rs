use bigdecimal::BigDecimal;
use sqlx::{PgPool};

use crate::{db::error::DbError, entities::{CreateOrderPayload, Product}};


pub async fn create(pool: &PgPool, mut order: CreateOrderPayload) -> Result<i64, DbError> {
    let mut tx = pool.begin()
        .await
        .map_err(|e| match e {
            default => DbError::Internal(default)
        })?;
    let mut total_price = BigDecimal::from(0);
    for i in 0..order.items.len() {
        let p = sqlx::query_as!(
            Product,
            r#"SELECT * FROM products WHERE id = $1 FOR UPDATE"#,
            order.items[i].product_id,
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => DbError::NotFound,
            default => DbError::Internal(default)
        })?;
        total_price += p.price.clone() * order.items[i].quantity;
        if p.stock_quantity < order.items[i].quantity {
            return Err(DbError::Conflict("not enough supply".to_string()));
        }
        let it = &mut order.items[i]; 
        it.unit_price = p.price 
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
   let product_ids: Vec<i64> = order.items.iter().map(|o| {
       o.product_id
   }).rev().collect();
   let unit_prices: Vec<BigDecimal> = order.items.iter().map(|o| {
       o.unit_price.clone()
   }).rev().collect();
   let quantities: Vec<i32> = order.items.iter().map(|o| {
       o.quantity
   }).rev().collect();
   
   sqlx::query!(r#"
        WITH data AS (
            SELECT 
            $1::BIGINT AS order_id,
            unnest($2::BIGINT[]) AS product_id,
            unnest($3::INT[]) AS quantity,
            unnest($4::NUMERIC(15,2)[]) AS unit_price
        )
        INSERT INTO order_items (order_id, product_id, quantity, unit_price)
        SELECT order_id, product_id, quantity, unit_price
        FROM data;
        "#,
        id,
        &product_ids,
        &quantities,
        &unit_prices)
       .execute(&mut *tx)
       .await
       .map_err(|e| match e {
           default => DbError::Internal(default)
       })?;

   sqlx::query!(
       r#"
         UPDATE products
         SET stock_quantity = products.stock_quantity + order_items.quantity
         FROM order_items
         WHERE order_items.order_id = $1 AND products.id = order_items.product_id"#,
         id)
     .execute(&mut *tx)
     .await
     .map_err(|e| match e {
         default => DbError::Internal(default)
     })?;

    tx.commit()
        .await
        .map_err(|e| match e {
            default => DbError::Internal(default)
        })?;

    Ok(id)
}
