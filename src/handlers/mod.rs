use serde::Serialize;
use axum::{response::IntoResponse, Json, http::StatusCode};

pub mod products;


#[derive(Serialize)]
pub struct Data {
    pub message: String,
}

pub struct AppError {
    pub status_code: StatusCode,
    pub data: Json<Data>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, self.data).into_response()
    }
}
