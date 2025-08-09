#![warn(clippy::pedantic)]
mod app_config;
mod task;

use std::sync::Arc;

use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::{Router, extract::State, routing::get};
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

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
        .route("/", get(index))
        .nest_service("/static", ServeDir::new("static"))
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

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    tasks: Vec<TaskDefinition>,
    valid_count: usize,
    invalid_count: usize,
}

async fn index(State(state): State<AppState>) -> IndexTemplate {
    let tasks_guard = state.tasks.read().await;
    let mut tasks_sorted = tasks_guard.clone();
    // stable partition: valid first, then invalid; within each group sort by name
    tasks_sorted.sort_by(|a, b| match (a.is_valid(), b.is_valid()) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.get_name().cmp(b.get_name()),
    });
    let (valid, invalid) = tasks_sorted.iter().fold((0usize, 0usize), |(v, i), t| {
        if t.is_valid() { (v + 1, i) } else { (v, i + 1) }
    });
    IndexTemplate {
        tasks: tasks_sorted,
        valid_count: valid,
        invalid_count: invalid,
    }
}

impl IntoResponse for IndexTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(rendered) => Html(rendered).into_response(),
            Err(err) => {
                tracing::error!(error = %err, "Failed to render index template");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Template rendering error",
                )
                    .into_response()
            }
        }
    }
}
