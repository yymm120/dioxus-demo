// Run with:
// ```bash
// cargo run --bin server --features server
// ```

use axum::*;
use axum::{response::Html, routing::get, Router};
use dioxus::prelude::*;
// use server_fn::axum::register_explicit;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.01:8080")
        .await
        .unwrap();

    let app = Router::new().route("/", get(handler));
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}