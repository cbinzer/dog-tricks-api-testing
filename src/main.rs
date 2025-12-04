use axum::serve;

use tokio::net::TcpListener;

use crate::trick_router::create_trick_router;

mod trick_handlers;
mod trick_models;
mod trick_repository;
mod trick_router;
mod trick_service;

#[tokio::main]
async fn main() {
    let trick_router = create_trick_router();

    let tcp_listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Server started on port 8080");
    serve(tcp_listener, trick_router).await.unwrap();
}
