use crate::task::Task;

pub fn display_labels(task: &Task) -> String {
    if task.tags.is_empty() {
        return "untagged".to_string();
    }

    task.tags.join(", ")
}
