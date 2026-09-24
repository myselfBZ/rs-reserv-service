use axum::{Router, routing::get};

async fn hello() ->  &'static str {
    
    return "Hello, world"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello));
    println!("we are up");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
