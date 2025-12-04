use crate::trick_models::{Trick, TrickCreateInput, TrickError, TrickReplaceInput};
use crate::trick_service::TrickService;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_trick(
    State(repo): State<Arc<TrickService>>,
    Json(input): Json<TrickCreateInput>,
) -> Result<(StatusCode, Json<Trick>), TrickError> {
    let new_trick = repo.create(input).await?;
    Ok((StatusCode::CREATED, Json(new_trick)))
}

pub async fn find_tricks(State(repo): State<Arc<TrickService>>) -> Json<Vec<Trick>> {
    let all_tricks = repo.find_all().await;
    Json(all_tricks)
}

pub async fn find_trick_by_id(
    State(repo): State<Arc<TrickService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Trick>, TrickError> {
    let trick = repo.find_by_id(id).await?;
    Ok(Json(trick))
}

pub async fn replace_trick(
    State(repo): State<Arc<TrickService>>,
    Path(id): Path<Uuid>,
    Json(input): Json<TrickReplaceInput>,
) -> Result<Json<Trick>, TrickError> {
    let replaced_trick = repo.replace(id, input).await?;
    Ok(Json(replaced_trick))
}

pub async fn delete_trick(State(repo): State<Arc<TrickService>>, Path(id): Path<Uuid>) {
    repo.delete_by_id(id).await
}
