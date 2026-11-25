//! 启动服务：cargo run --bin axum_server
//! 运行测试：cargo test --bin axum_server

mod handlers;
mod routes;
#[cfg(test)]
mod tests;

use tokio::net::TcpListener;

const DEFAULT_ADDR: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() {
    let addr = std::env::var("AXUM_ADDR").unwrap_or_else(|_| DEFAULT_ADDR.to_string());
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Axum server listening on http://{addr}");

    axum::serve(listener, routes::build_router()).await.unwrap();
}
