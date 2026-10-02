use bigdecimal::BigDecimal;
use sqlx::PgPool;

use crate::{db::{error::DbError, order::{self, CreateOrderParam}, products}, entities::{CreateOrderPayload}};


// TODO change DbError to ServiceError
pub async fn create(pool: &PgPool, mut payload: CreateOrderPayload) -> Result<i64, DbError> {
    let mut tx = pool.begin()
        .await
        .map_err(|e| DbError::Internal(e))?;
    let product_ids: Vec<i64> = payload.items.iter().map(
        |o| { o.product_id }
    ).rev().collect();
    let products = products::get_many(&mut *tx, &product_ids).await?;
    let mut total = BigDecimal::from(0);
    for i in 0..payload.items.len() {
        for p in products.iter() {
            if p.id == payload.items[i].product_id {
                if p.stock_quantity < payload.items[i].quantity {
                    return Err(DbError::Conflict("not enough stock".to_string()))
                }
                payload.items[i].unit_price = p.price.clone();
                total += p.price.clone() * payload.items[i].quantity; 
                break
            }
            // invalid product id in the order_item
            return Err(DbError::NotFound)
        }
    }
    let id = order::create_order(
        &mut *tx, 
        CreateOrderParam { user_id: payload.user_id, total_price: total},
        )
        .await?;

    order::create_items(&mut *tx, id, &payload.items).await?;
    products::update_stock(&mut *tx, id).await?;

    tx.commit()
        .await
        .map_err(|e| DbError::Internal(e))?;
    Ok(id)
}
