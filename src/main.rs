#![warn(clippy::pedantic)]
mod config;
use config::{APP_NAME, APP_PORT};

use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{APP_PORT}"))
        .await
        .unwrap();

    tracing::debug!(
        "{APP_NAME} is listening on {}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Sir, I'm afraid that your breakfast will be four minutes late."
}
