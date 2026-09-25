
pub enum DbError {
    NotFound,
    Conflict(String),
    Internal(sqlx::Error)
}
