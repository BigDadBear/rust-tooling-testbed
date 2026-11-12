use crate::task::Task;

pub fn completion_percentage(tasks: &[Task]) -> f64 {
    if tasks.is_empty() {
        return 0.0;
    }

    let completed = tasks.iter().filter(|task| task.done).count();
    completed as f64 / tasks.len() as f64 * 100.0
}
