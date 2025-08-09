//! This module contains configuration constants used throughout the application.

/// The name of the application.
pub const APP_NAME: &str = "Woodhouse";

/// The port on which the app server will listen for incoming connections.
pub const APP_PORT: u16 = 4242;

/// The default directory (relative to the working directory) to load tasks from.
pub const DEFAULT_TASKS_DIRECTORY: &str = "tasks";

pub struct AppConfig {
    pub app_name: String,
    pub port: u16,
    pub tasks_directory: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let port = std::env::var("WOODHOUSE_PORT")
            .ok()
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(APP_PORT);

        let tasks_directory =
            std::env::var("WOODHOUSE_TASK_DIR").unwrap_or(DEFAULT_TASKS_DIRECTORY.to_string());

        Self {
            app_name: APP_NAME.to_string(),
            port,
            tasks_directory,
        }
    }
}
