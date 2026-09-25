use sqlx::PgPool;

use crate::entities::Product;
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

pub async fn create(pool: &PgPool, p: Product) -> Result<i64, DbError> {
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
