use std::sync::Arc;
use axum::{Json, extract::{State, Path}, http::StatusCode};
use crate::{
    db::{error::DbError, products}, entities::{CreateProductPayload, Product}, handlers::{AppError, Data}, state::AppState
};


pub async fn get_by_id(State(state): State<Arc<AppState>>, Path(id): Path<i64>) -> Result<Json<Product>, AppError> {
    let p = products::get_by_id(&state.pool, id)
        .await
        .map_err(|err| match err {
            DbError::NotFound => AppError{
                status_code: StatusCode::NOT_FOUND, 
                data: Json(Data{message: "product not found".to_string()})
            },
            _ => AppError { 
                status_code: StatusCode::INTERNAL_SERVER_ERROR, 
                data: Json(Data{ message: "internal server error".to_string()})
            }
        })?;
    Ok(Json(p))
}

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
