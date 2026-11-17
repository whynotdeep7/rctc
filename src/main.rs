use axum::{Router, routing::get, serve::Listener};
async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main(){
    let app=Router::new().route("/health",get(health));
    let listener= tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("listening to :8080");

    axum::serve(listener, app).await.unwrap();
}