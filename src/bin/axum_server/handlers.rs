use axum::{Json, extract::Path};
use serde::Deserialize;
use serde_json::{Value, json};

pub async fn greet(Path(name): Path<String>) -> Json<Value> {
    Json(json!({ "message": format!("Hello, {name}!") }))
}

#[derive(Deserialize)]
pub struct GreetingRequest {
    message: String,
}

pub async fn greet_with_message(
    Path(name): Path<String>,
    Json(request): Json<GreetingRequest>,
) -> Json<Value> {
    Json(json!({ "message": format!("{}, {}!", request.message, name) }))
}
