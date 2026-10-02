use sqlx::PgPool;

use crate::entities::{CreateProductPayload, Product};
use crate::db::error::DbError;

pub async fn get_by_id(pool: &PgPool, id: i64) -> Result<Product, DbError> {
    sqlx::query_as!(
        Product,
        r#"
            SELECT
                id,
                name,
                price,
                stock_quantity
            FROM products WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => DbError::NotFound,
        other => DbError::Internal(other),
    })
}

pub async fn get_many<'a, T:sqlx::PgExecutor<'a>>(db: T, product_ids: &[i64]) -> Result<Vec<Product>, DbError> {
    let products = sqlx::query_as!(
        Product,
        r#"
            SELECT * FROM products WHERE id = ANY($1::BIGINT[])
        "#,
        &product_ids
    )
     .fetch_all(db)
     .await
     .map_err(|e| DbError::Internal(e))?;
    Ok(products)
}

pub async fn update_stock<'a, T:sqlx::PgExecutor<'a>>(db: T, order_id: i64) -> Result<(), DbError> {
    sqlx::query!(
        r#"
         UPDATE products
         SET stock_quantity = products.stock_quantity - order_items.quantity
         FROM order_items
         WHERE order_items.order_id = $1 AND products.id = order_items.product_id"#,
         order_id)
        .execute(db)
        .await
        .map_err(|e| match e {
            default => DbError::Internal(default)
        })?;
    Ok(())
}

pub async fn create(pool: &PgPool, p: CreateProductPayload) -> Result<i64, DbError> {
    sqlx::query_scalar!(
        r#"INSERT INTO 
            products(name, price, stock_quantity)
           VALUES($1, $2, $3) RETURNING id
        "#,
        p.name,
        p.price,
        p.stock_quantity
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        default => DbError::Internal(default)
    }) 
}
