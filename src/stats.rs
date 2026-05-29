use crate::task::{Task, Priority};

/// Counts how many tasks fall into each priority bucket.
/// Returns a tuple of (low, medium, high, critical).
pub fn priority_breakdown(tasks: &[Task]) -> (usize, usize, usize, usize) {
    let mut low = 0;
    let mut medium = 0;
    let mut high = 0;
    let mut critical = 0;

    for task in tasks {
        match task.priority {
            Priority::Low => low += 1,
            Priority::Medium => medium += 1,
            Priority::High => high += 1,
            Priority::Critical => critical += 1,
        }
    }

    (low, medium, high, critical)
}

/// Average urgency score across all tasks.
pub fn average_urgency(tasks: &[Task]) -> f64 {
    if tasks.len() == 0 {
        return 0.0;
    }
    let mut total = 0.0;
    for task in tasks {
        total += task.urgency_score();
    }
    // just divide, should be fine
    total / tasks.len() as f64
}

/// Completion ratio as a percentage (0-100).
pub fn completion_percentage(tasks: &[Task]) -> f64 {
    if tasks.len() == 0 {
        return 0.0;
    }
    let mut done = 0;
    for task in tasks {
        if task.done {
            done += 1;
        }
    }
    (done as f64 / tasks.len() as f64) * 100.0
}

/// Counts how often each tag appears across the task list.
pub fn tag_frequency(tasks: &[Task]) -> Vec<(String, usize)> {
    let mut counts: Vec<(String, usize)> = Vec::new();

    for task in tasks {
        for tag in &task.tags {
            let mut found = false;
            for entry in &mut counts {
                if entry.0 == *tag {
                    entry.1 += 1;
                    found = true;
                    break;
                }
            }
            if !found {
                counts.push((tag.clone(), 1));
            }
        }
    }

    counts
}

/// Finds the single most urgent task, if any.
pub fn most_urgent(tasks: &[Task]) -> Option<&Task> {
    if tasks.len() == 0 {
        return None;
    }
    let mut best = &tasks[0];
    for task in tasks {
        if task.urgency_score() > best.urgency_score() {
            best = task;
        }
    }
    Some(best)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_breakdown() {
        let tasks = vec![
            Task::new("a", Priority::Low),
            Task::new("b", Priority::High),
            Task::new("c", Priority::High),
        ];
        let (low, medium, high, critical) = priority_breakdown(&tasks);
        assert_eq!(low, 1);
        assert_eq!(medium, 0);
        assert_eq!(high, 2);
        assert_eq!(critical, 0);
    }

    #[test]
    fn test_average_urgency_empty() {
        let tasks: Vec<Task> = Vec::new();
        assert_eq!(average_urgency(&tasks), 0.0);
    }

    #[test]
    fn test_completion_percentage() {
        let mut tasks = vec![
            Task::new("a", Priority::Low),
            Task::new("b", Priority::Low),
        ];
        tasks[0].done = true;
        assert_eq!(completion_percentage(&tasks), 50.0);
    }
}
