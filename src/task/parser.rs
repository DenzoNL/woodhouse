use crate::task::{Task, TaskDefinition, TaskError};

pub async fn parse_task_file(file_path: &str) -> TaskDefinition {
    let mut errors = Vec::new();

    let file_contents = match read_task_file(file_path).await {
        Ok(content) => content,
        Err(e) => {
            errors.push(e);
            return TaskDefinition {
                file_path: file_path.to_string(),
                file_contents: None,
                task: None,
                errors,
            };
        }
    };

    let task = match deserialize_task(&file_contents) {
        Ok(task) => task,
        Err(e) => {
            errors.push(e);
            return TaskDefinition {
                file_path: file_path.to_string(),
                file_contents: Some(file_contents),
                task: None,
                errors,
            };
        }
    };

    TaskDefinition {
        file_path: file_path.to_string(),
        file_contents: Some(file_contents),
        task: Some(task),
        errors,
    }
}

pub fn deserialize_task(content: &str) -> Result<Task, TaskError> {
    let task: Task = match toml::from_str(content) {
        Ok(task) => task,
        Err(e) => {
            return Err(TaskError::InvalidToml(format!(
                "Failed to parse TOML: {}",
                e
            )));
        }
    };

    Ok(task)
}

async fn read_task_file(file_path: &str) -> Result<String, TaskError> {
    match tokio::fs::read_to_string(file_path).await {
        Ok(content) => Ok(content),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Err(TaskError::FileNotFound(file_path.to_string()))
        }
        Err(e) => Err(TaskError::FileReadError(format!("{}: {}", file_path, e))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TASK_CONTENT: &str = r#"name = "Test task"
env = "test"
command = "echo"
args = ["Hello, World!"]
schedule = "*/5 * * *""#;

    #[tokio::test]
    async fn test_read_task_file() {
        let file_path = "tests/test_task.toml";
        let content = read_task_file(file_path).await.unwrap();

        assert_eq!(content, TEST_TASK_CONTENT);
    }

    #[tokio::test]
    async fn test_deserialize_task() {
        let task = deserialize_task(TEST_TASK_CONTENT).unwrap();

        assert_eq!(task.name, "Test task");
        assert_eq!(task.env, Some("test".to_string()));
        assert_eq!(task.command, "echo");
        assert_eq!(task.args, vec!["Hello, World!"]);
        assert_eq!(task.schedule, Some("*/5 * * *".to_string()));
        assert!(task.environment.is_none());
    }

    #[tokio::test]
    async fn test_parse_task_file() {
        let file_path = "tests/test_task.toml";
        let task_definition = parse_task_file(file_path).await;

        assert!(task_definition.is_valid());
        // print errors if any
        if !task_definition.errors.is_empty() {
            for error in &task_definition.errors {
                eprintln!("Error: {}", error);
            }
        }

        let task = task_definition.task.unwrap();

        assert_eq!(task.name, "Test task");
        assert_eq!(task.env, Some("test".to_string()));
        assert_eq!(task.command, "echo");
        assert_eq!(task.args, vec!["Hello, World!"]);
        assert_eq!(task.schedule, Some("*/5 * * *".to_string()));
        assert!(task.environment.is_none());
    }
}
