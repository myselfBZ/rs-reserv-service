use crate::{db::{error::DbError, order}, entities::{CreateOrderPayload}, handlers::{AppError, Data}, state::AppState};
use std::sync::Arc;
use axum::{Json, extract::State, http::StatusCode};

pub async fn create(
    State(state):State<Arc<AppState>>,
    Json(payload): Json<CreateOrderPayload>,
    ) -> Result<Json<i64>, AppError> {
    let id = order::create(&state.pool, payload)
        .await
        .map_err(|err| match err {
            DbError::NotFound => AppError{
                status_code: StatusCode::NOT_FOUND,
                data: Json(Data{message: "product not found".to_string()})
            },
            DbError::Conflict(s) => AppError { 
                status_code: StatusCode::CONFLICT, 
                data: Json(Data{message: s})
            },
            // TODO, Log the damn error dude wtf?
            DbError::Internal(_) => AppError { 
                status_code: StatusCode::INTERNAL_SERVER_ERROR, 
                data: Json(Data{message: "internal server error".to_string()}) 
            }
        })?;
    Ok(Json(id))
}
