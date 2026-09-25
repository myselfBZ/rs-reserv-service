use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


#[derive(Serialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize)]
pub struct OrderItem {
    pub order_id: i64,
    pub product_id: i64,
    pub unit_price: BigDecimal,
    pub quantity: i32
}

#[derive(Serialize)]
pub struct Order {
    pub id: i64,
    pub user_id: i64,
    pub placed_at: NaiveDateTime,
    pub total_price: BigDecimal,
    pub items: Vec<OrderItem>
}

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub price: BigDecimal,
    pub stock_quantity: i32
}
