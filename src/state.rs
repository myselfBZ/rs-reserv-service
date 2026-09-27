use sqlx::PgPool;

pub struct AppState {
    pool: PgPool
}
