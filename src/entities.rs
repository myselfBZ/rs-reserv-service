use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub price: BigDecimal,
    pub stock_quantity: i32
}
