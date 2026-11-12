use crate::task::Task;

pub fn next_pending(tasks: &[Task]) -> Option<&Task> {
    tasks
        .iter()
        .filter(|task| !task.done)
        .max_by(|left, right| left.urgency_score().total_cmp(&right.urgency_score()))
}
