use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Method, Request, Response, StatusCode, header},
};
use serde_json::{Value, json};
use tower::ServiceExt;

use crate::routes::build_router;

const MAX_BODY_BYTES: usize = 1024;

async fn send_request(app: Router, method: Method, uri: &str) -> Response<Body> {
    let request = Request::builder()
        .method(method)
        .uri(uri)
        .body(Body::empty())
        .unwrap();

    app.oneshot(request).await.unwrap()
}

async fn read_json(response: Response<Body>) -> Value {
    let bytes = to_bytes(response.into_body(), MAX_BODY_BYTES)
        .await
        .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn get_greeting_returns_json() {
    let response = send_request(build_router(), Method::GET, "/hello/Axum").await;

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()[header::CONTENT_TYPE], "application/json");
    assert_eq!(
        read_json(response).await,
        json!({ "message": "Hello, Axum!" })
    );
}

#[tokio::test]
async fn unknown_route_returns_not_found() {
    let response = send_request(build_router(), Method::GET, "/missing").await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn post_greeting_uses_json_message() {
    let request = Request::builder()
        .method(Method::POST)
        .uri("/hello/Axum")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(r#"{"message":"Hi"}"#))
        .unwrap();
    let response = build_router().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(read_json(response).await, json!({ "message": "Hi, Axum!" }));
}

#[tokio::test]
async fn unsupported_method_returns_method_not_allowed() {
    let response = send_request(build_router(), Method::PUT, "/hello/Axum").await;

    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
}
