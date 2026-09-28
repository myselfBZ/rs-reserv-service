use std::sync::Arc;
use dotenvy::dotenv;
use axum::{Router, routing::{get, post}};
use rs_reserv_service::{handlers::{orders, products}, state::AppState};
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("where's the database?");
    let pg_pool = PgPool::connect(&db_url).await.expect("HUH?");
    let state = Arc::new(AppState{
        pool: pg_pool
    });
    let app = Router::new()
        .route("/products", post(products::create))
        .route("/products/{id}", get(products::get_by_id))
        .route("/orders", post(orders::create))
        .with_state(state);
    println!("we are up");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
