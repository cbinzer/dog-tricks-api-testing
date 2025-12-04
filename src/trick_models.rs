use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Instruction {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Trick {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub instructions: Vec<Instruction>,
}

#[derive(Serialize, Deserialize)]
pub struct TrickCreateInput {
    pub title: String,
    pub description: String,
    pub instructions: Vec<Instruction>,
}

#[derive(Serialize, Deserialize)]
pub struct TrickReplaceInput {
    pub title: String,
    pub description: String,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub enum TrickError {
    NotFound(Uuid),
    Validation(String),
}

impl Error for TrickError {}

impl Display for TrickError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrickError::NotFound(id) => write!(f, "Trick with id {} not found", id),
            TrickError::Validation(msg) => write!(f, "Validation failed: {}", msg),
        }
    }
}

impl IntoResponse for TrickError {
    fn into_response(self) -> Response {
        let (status_code, message) = match self {
            TrickError::NotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("Trick with id {id} not found"),
            ),
            TrickError::Validation(message) => (StatusCode::BAD_REQUEST, message),
        };

        let json_error = Json(ApiError {
            status_code: status_code.as_u16(),
            message,
        });

        (status_code, json_error).into_response()
    }
}

#[derive(Serialize)]
struct ApiError {
    status_code: u16,
    message: String,
}
