use crate::trick_handlers::{
    create_trick, delete_trick, find_trick_by_id, find_tricks, replace_trick,
};
use crate::trick_repository::TrickRepository;
use axum::routing::get;
use axum::{Router, serve};

use std::sync::Arc;
use tokio::net::TcpListener;
use crate::trick_service::TrickService;

mod trick_handlers;
mod trick_models;
mod trick_repository;
mod trick_service;

#[tokio::main]
async fn main() {
    let service = Arc::new(TrickService::new(TrickRepository::new()));
    let router = Router::new()
        .route("/tricks", get(find_tricks).post(create_trick))
        .route(
            "/tricks/{id}",
            get(find_trick_by_id)
                .put(replace_trick)
                .delete(delete_trick),
        )
        .with_state(service);

    let tcp_listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Server started on port 8080");
    serve(tcp_listener, router).await.unwrap();
}
