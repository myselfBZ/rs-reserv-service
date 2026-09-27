use std::sync::Arc;
use axum::{Json, extract::State, http::StatusCode};
use bigdecimal::BigDecimal;
use serde::Deserialize;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateProductPayload {
    name: String,
    price: BigDecimal,
    stock_quantity: i64
}

pub async fn create(
    State(state): State<Arc<AppState>>, 
    Json(payload): Json<CreateProductPayload>
    ) -> StatusCode {
    StatusCode::OK
}
