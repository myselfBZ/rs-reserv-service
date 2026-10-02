use bigdecimal::BigDecimal;
use crate::{db::error::DbError, entities::{OrderItemPayload}};

pub struct CreateOrderParam {
    pub user_id: i64,
    pub total_price: BigDecimal
}

pub async fn create_items<'a, T:sqlx::PgExecutor<'a>>(db: T, order_id: i64, items: &[OrderItemPayload]) -> Result<(), DbError> {
   let product_ids: Vec<i64> = items.iter().map(|o| {
       o.product_id
   }).rev().collect();
   let unit_prices: Vec<BigDecimal> = items.iter().map(|o| {
       o.unit_price.clone()
   }).rev().collect();
   let quantities: Vec<i32> = items.iter().map(|o| {
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
        order_id,
        &product_ids,
        &quantities,
        &unit_prices)
       .execute(db)
       .await
       .map_err(|e| match e {
           default => DbError::Internal(default)
       })?;

   Ok(())
}

pub async fn create_order<'a, T: sqlx::PgExecutor<'a>>(db: T, o: CreateOrderParam) -> Result<i64, DbError> {
    let id = sqlx::query_scalar!(r#"
        INSERT INTO orders(user_id, total_price) VALUES($1, $2) RETURNING id
        "#,
        o.user_id,
        o.total_price
        )
        .fetch_one(db)
        .await
        .map_err(|e| match e {
            default => DbError::Internal(default)
        })?;
    Ok(id)
}
