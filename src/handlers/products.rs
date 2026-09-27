use std::sync::Arc;
use axum::{Json, extract::State, http::StatusCode};
use crate::{
    db::products, 
    entities::{CreateProductPayload}, 
    handlers::AppError, 
    state::AppState,
    handlers::Data
};

pub async fn create(
    State(state): State<Arc<AppState>>, 
    Json(payload): Json<CreateProductPayload>
    ) -> Result<Json<i64>, AppError> {
    let id = products::create(&state.pool, payload)
    .await
    .map_err(|_| AppError {
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        data: Json(Data { message: "internal server error".to_string() })
    })?;
    Ok(Json(id))
}
