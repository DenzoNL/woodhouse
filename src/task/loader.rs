use std::io::Result;
use std::path::{Path, PathBuf};

use tokio::fs;

use crate::task::TaskDefinition;
use crate::task::parser::parse_task_file;

/// Return (non‑recursive) list of `.toml` files in `directory` (case‑insensitive extension).
/// Missing directory -> Ok(empty).
async fn list_task_files<P: AsRef<Path>>(directory: P) -> Result<Vec<PathBuf>> {
    let directory = directory.as_ref();
    if !directory.exists() {
        return Ok(Vec::new());
    }

    let mut entries = fs::read_dir(directory).await?;
    let mut file_paths = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        let ft = entry.file_type().await?;
        if !ft.is_file() {
            continue;
        }
        let path = entry.path();
        let ext_ok = path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| e.eq_ignore_ascii_case("toml"));
        if ext_ok {
            file_paths.push(path);
        }
    }

    file_paths.sort();
    Ok(file_paths)
}

pub async fn load_tasks_from_directory<P: AsRef<Path>>(
    directory: P,
) -> Result<Vec<TaskDefinition>> {
    let directory = directory.as_ref();
    let files = list_task_files(directory).await?;
    let total = files.len();
    let mut tasks = Vec::with_capacity(total);

    if total == 0 {
        tracing::info!("No task files found in directory: {}", directory.display());
        return Ok(tasks);
    }

    tracing::info!(
        "Found {} task file(s) in directory: {}",
        total,
        directory.display()
    );

    for (index, file_path) in files.iter().enumerate() {
        let ordinal = index + 1;
        tracing::info!(
            "[{}/{}] Parsing task file: {}",
            ordinal,
            total,
            file_path.display()
        );

        let task_def = parse_task_file(file_path).await;

        if task_def.errors.is_empty() {
            tracing::info!(
                "[{}/{}] Loaded task '{}' ({})",
                ordinal,
                total,
                task_def.get_name(),
                file_path.display()
            );
        } else {
            for err in &task_def.errors {
                tracing::warn!(
                    "[{}/{}] Error in task file '{}': {}",
                    ordinal,
                    total,
                    file_path.display(),
                    err
                );
            }
        }
        tasks.push(task_def);
    }

    let success = tasks.iter().filter(|t| t.is_valid()).count();
    let failed = total - success;

    if failed == 0 {
        tracing::info!(
            "Successfully loaded all {} task file(s) from {}",
            success,
            directory.display()
        );
    } else {
        tracing::warn!(
            "[{}/{}] task file(s) successfully loaded ({} with errors) from {}",
            success,
            total,
            failed,
            directory.display()
        );
    }

    Ok(tasks)
}

mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_task_files() {
        let files = list_task_files("tasks").await.unwrap();
        assert!(!files.is_empty());
        for file in &files {
            assert!(file.is_file());
        }
    }

    #[tokio::test]
    async fn test_load_tasks_from_directory() {
        let tasks = load_tasks_from_directory("tasks").await.unwrap();
        assert!(!tasks.is_empty(), "expected at least one task file");

        for td in &tasks {
            if td.is_valid() {
                assert!(td.file_contents.is_some(), "valid task missing contents");
                assert!(td.task.is_some(), "valid task missing parsed task");
                assert!(td.errors.is_empty(), "valid task has unexpected errors");
            } else {
                assert!(td.task.is_none(), "invalid task unexpectedly parsed");
                assert!(!td.errors.is_empty(), "invalid task missing errors");
            }
        }
    }

    #[tokio::test]
    async fn test_load_tasks_from_nonexistent_directory() {
        let tasks = load_tasks_from_directory("nonexistent").await.unwrap();
        assert!(tasks.is_empty());
    }
}
