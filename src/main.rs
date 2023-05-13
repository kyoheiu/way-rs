use axum::debug_handler;
use axum::response::Html;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/health", get(health));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:9090".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[debug_handler]
async fn health() -> Html<&'static str> {
    Html("Hello, developer.")
}
