use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use dog_tricks_api_testing::trick_models::{ApiError, Trick};
use dog_tricks_api_testing::trick_router::create_trick_router;
use http_body_util::BodyExt;
use hyper::body::Bytes;
use tower::ServiceExt;

#[tokio::test]
async fn should_create_a_trick() {
    let router = create_trick_router();

    let payload = serde_json::json!({
        "title": "Sit",
        "description": "Sit...",
        "instructions": []
    });

    let response = router
        .oneshot(
            Request::post("/tricks")
                .header("Content-Type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = response.into_body().collect().await.unwrap();
    let bytes: Bytes = body.to_bytes();
    let trick: Trick = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(trick.title, "Sit");
    assert_eq!(trick.description, "Sit...");
    assert!(trick.instructions.is_empty());
}

#[tokio::test]
async fn should_return_400_when_title_is_empty() {
    let router = create_trick_router();

    let payload = serde_json::json!({
        "title": "",
        "description": "Sit...",
        "instructions": []
    });

    let response = router
        .oneshot(
            Request::post("/tricks")
                .header("Content-Type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response.into_body().collect().await.unwrap();
    let bytes: Bytes = body.to_bytes();
    let api_error: ApiError = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(api_error.status_code, StatusCode::BAD_REQUEST.as_u16());
    assert_eq!(api_error.message, "title must not be empty");
}
