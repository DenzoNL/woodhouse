use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Task {
    pub name: String,
    pub env: Option<String>,
    pub command: String,
    pub args: Option<Vec<String>>,
    pub environment: Option<HashMap<String, String>>,
    pub schedule: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct TaskDefinition {
    pub file_path: PathBuf,
    pub file_contents: Option<String>,
    pub task: Option<Task>,
    pub errors: Vec<TaskError>,
}

impl TaskDefinition {
    pub fn is_valid(&self) -> bool {
        self.task.is_some() && self.errors.is_empty()
    }

    pub fn get_name(&self) -> &str {
        if let Some(task) = &self.task {
            return task.name.as_str();
        }
        self.file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
    }

    pub fn status_label(&self) -> String {
        if self.errors.is_empty() {
            "Valid".to_string()
        } else {
            format!("Invalid ({} errors)", self.errors.len())
        }
    }

    pub fn file_basename(&self) -> &str {
        self.file_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
    }

    pub fn schedule(&self) -> Option<&str> {
        self.task.as_ref().and_then(|t| t.schedule.as_deref())
    }

    pub fn environment_label(&self) -> Option<&str> {
        self.task.as_ref().and_then(|t| t.env.as_deref())
    }

    pub fn description(&self) -> Option<&str> {
        self.task.as_ref().and_then(|t| t.description.as_deref())
    }
}

// Placeholder for future runtime metadata (next run, last run, running state)
#[derive(Debug, Serialize, Clone, Default)]
pub struct TaskRuntimeStatus {
    pub last_run: Option<String>,
    pub next_run: Option<String>,
    pub is_running: bool,
}

#[derive(Debug, Serialize, Clone)]
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
