use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Task {
    pub name: String,
    pub env: Option<String>,
    pub command: String,
    pub args: Vec<String>,
    pub environment: Option<HashMap<String, String>>,
    pub schedule: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TaskDefinition {
    pub file_path: String,
    pub file_contents: Option<String>,
    pub task: Option<Task>,
    pub errors: Vec<TaskError>,
}

impl TaskDefinition {
    pub fn is_valid(&self) -> bool {
        self.task.is_some() && self.errors.is_empty()
    }
}

#[derive(Debug, Clone)]
pub enum TaskError {
    FileNotFound(String),
    FileReadError(String),
    InvalidToml(String),
    InvalidCron(String),
    MissingField(String),
    InvalidCommand(String),
}

impl std::fmt::Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskError::FileNotFound(path) => write!(f, "File not found: {}", path),
            TaskError::FileReadError(msg) => write!(f, "Failed to read file: {}", msg),
            TaskError::InvalidToml(msg) => write!(f, "Invalid TOML: {}", msg),
            TaskError::InvalidCron(msg) => write!(f, "Invalid cron expression: {}", msg),
            TaskError::MissingField(field) => write!(f, "Missing required field: {}", field),
            TaskError::InvalidCommand(msg) => write!(f, "Invalid command: {}", msg),
        }
    }
}

impl std::error::Error for TaskError {}
