use axum::{Router, routing::get};

use crate::handlers;

pub fn build_router() -> Router {
    Router::new().route(
        "/hello/{name}",
        get(handlers::greet).post(handlers::greet_with_message),
    )
}
