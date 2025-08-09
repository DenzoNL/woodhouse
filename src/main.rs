#![warn(clippy::pedantic)]
mod app_config;

mod task;

use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::get};
use tokio::sync::RwLock;

use crate::{app_config::AppConfig, task::TaskDefinition};

#[derive(Clone)]
struct AppState {
    tasks: Arc<RwLock<Vec<TaskDefinition>>>,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let cfg = AppConfig::from_env();

    let loaded_tasks = match task::loader::load_tasks_from_directory(&cfg.tasks_directory).await {
        Ok(tasks) => tasks,
        Err(e) => {
            tracing::error!("Failed to load tasks: {e}");
            Vec::new()
        }
    };

    let state = AppState {
        tasks: Arc::new(RwLock::new(loaded_tasks)),
    };

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/tasks", get(list_tasks))
        .with_state(state.clone());

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", cfg.port))
        .await
        .unwrap();

    tracing::debug!(
        "{} is listening on {}",
        cfg.app_name,
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Sir, I'm afraid that your breakfast will be four minutes late."
}

async fn list_tasks(State(state): State<AppState>) -> Json<Vec<TaskDefinition>> {
    let tasks = state.tasks.read().await;
    Json(tasks.clone())
}
