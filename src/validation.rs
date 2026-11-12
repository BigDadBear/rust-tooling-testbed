use crate::task::Task;

pub fn validate_task(task: &Task) -> Result<(), String> {
    if task.title.trim().is_empty() {
        return Err("task title cannot be empty".to_string());
    }

    Ok(())
}
