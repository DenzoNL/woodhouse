use std::path::{Path, PathBuf};

use crate::task::{Task, TaskDefinition, TaskError};

pub async fn parse_task_file<P: AsRef<Path>>(path: P) -> TaskDefinition {
    let path_ref = path.as_ref();
    let path_buf: PathBuf = path_ref.to_path_buf();
    let mut errors = Vec::new();

    // Read
    let file_contents = match read_task_file(&path_buf).await {
        Ok(c) => c,
        Err(e) => {
            errors.push(e);
            return TaskDefinition {
                file_path: path_buf,
                file_contents: None,
                task: None,
                errors,
            };
        }
    };

    // Deserialize
    let task = match deserialize_task(&file_contents) {
        Ok(t) => t,
        Err(e) => {
            errors.push(e);
            return TaskDefinition {
                file_path: path_buf,
                file_contents: Some(file_contents),
                task: None,
                errors,
            };
        }
    };

    TaskDefinition {
        file_path: path_buf,
        file_contents: Some(file_contents),
        task: Some(task),
        errors,
    }
}

fn deserialize_task(content: &str) -> Result<Task, TaskError> {
    toml::from_str::<Task>(content)
        .map_err(|e| TaskError::InvalidToml(format!("Failed to parse TOML: {e}")))
}

async fn read_task_file<P: AsRef<Path>>(path: P) -> Result<String, TaskError> {
    let p = path.as_ref();
    tokio::fs::read_to_string(p).await.map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            TaskError::FileNotFound(p.display().to_string())
        } else {
            TaskError::FileReadError(format!("{}: {e}", p.display()))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str = r#"name = "Test task"
env = "test"
command = "echo"
args = ["Hello, World!"]
schedule = "*/5 * * *""#;

    #[tokio::test]
    async fn parse_valid_inline() {
        let task = deserialize_task(VALID).expect("should parse");
        assert_eq!(task.name, "Test task");
    }

    #[tokio::test]
    async fn missing_file_error() {
        let td = parse_task_file("tasks/does_not_exist.toml").await;
        assert!(!td.is_valid());
        assert!(
            td.errors
                .iter()
                .any(|e| matches!(e, TaskError::FileNotFound(_)))
        );
    }

    #[tokio::test]
    async fn invalid_toml_error() {
        let bad = "name = ";
        let res = deserialize_task(bad);
        assert!(matches!(res, Err(TaskError::InvalidToml(_))));
    }
}
