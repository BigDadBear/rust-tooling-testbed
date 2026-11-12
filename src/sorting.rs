use crate::task::Task;

pub fn by_title(tasks: &[Task]) -> Vec<&Task> {
    let mut sorted: Vec<&Task> = tasks.iter().collect();
    sorted.sort_by_key(|task| task.title.to_lowercase());
    sorted
}
