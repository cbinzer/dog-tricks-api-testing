use std::sync::Arc;

use crate::trick_handlers::{
    create_trick, delete_trick, find_trick_by_id, find_tricks, replace_trick,
};
use crate::trick_repository::TrickRepository;
use crate::trick_service::TrickService;

use axum::{Router, routing::get};

pub fn create_trick_router() -> Router {
    let service = Arc::new(TrickService::new(Arc::new(TrickRepository::new())));
    Router::new()
        .route("/tricks", get(find_tricks).post(create_trick))
        .route(
            "/tricks/{id}",
            get(find_trick_by_id)
                .put(replace_trick)
                .delete(delete_trick),
        )
        .with_state(service)
}
