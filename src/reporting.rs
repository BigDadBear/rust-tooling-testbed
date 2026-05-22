use crate::task::{Task, Priority};

/// Generate a simple text report of tasks
pub fn summary_report(tasks: &[Task]) -> String {
    let total = tasks.len();
    let done = tasks.iter().filter(|t| t.done).count();
    let mut out = format!("Report: {} tasks ({} done)\n", total, done);

    // group by priority manually
    for priority in &[Priority::Critical, Priority::High, Priority::Medium, Priority::Low] {
        let count = tasks.iter().filter(|t| t.priority == *priority).count();
        if count > 0 {
            out.push_str(&format!("  {:?}: {}\n", priority, count));
        }
    }
    out
}

/// Get top N tasks sorted by urgency
pub fn top_urgent(tasks: &[Task], n: usize) -> Vec<&Task> {
    let mut sorted: Vec<&Task> = tasks.iter().filter(|t| !t.done).collect();
    sorted.sort_by(|a, b| b.urgency_score().partial_cmp(&a.urgency_score()).unwrap());
    sorted.truncate(n);
    sorted
}

/// Count tasks per tag, returned as (tag, count) pairs
pub fn tag_distribution(tasks: &[Task]) -> Vec<(String, usize)> {
    let mut tags: Vec<(String, usize)> = Vec::new();
    for task in tasks {
        for tag in &task.tags {
            let mut found = false;
            for entry in &mut tags {
                if entry.0 == *tag {
                    entry.1 += 1;
                    found = true;
                }
            }
            if !found {
                tags.push((tag.clone(), 1));
            }
        }
    }
    tags
}

/// Completion rate as a value between 0.0 and 1.0
pub fn completion_rate(tasks: &[Task]) -> f64 {
    if tasks.is_empty() {
        return 0.0;
    }
    let done = tasks.iter().filter(|t| t.done).count();
    done as f64 / tasks.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tasks() -> Vec<Task> {
        vec![
            Task::new("Deploy fix", Priority::Critical),
            Task::new("Update docs", Priority::Low),
            Task::new("Review PR", Priority::Medium),
        ]
    }

    #[test]
    fn test_summary_report_header() {
        let tasks = sample_tasks();
        let report = summary_report(&tasks);
        assert!(report.contains("3 tasks (0 done)"));
    }

    #[test]
    fn test_top_urgent_limit() {
        let tasks = sample_tasks();
        let top = top_urgent(&tasks, 2);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].title, "Deploy fix");
    }

    #[test]
    fn test_tag_distribution_empty() {
        let tasks = sample_tasks();
        let dist = tag_distribution(&tasks);
        assert_eq!(dist.len(), 0);
    }

    #[test]
    fn test_completion_rate_none_done() {
        let tasks = sample_tasks();
        assert_eq!(completion_rate(&tasks), 0.0);
    }

    #[test]
    fn test_completion_rate_all_done() {
        let mut tasks = sample_tasks();
        for t in &mut tasks {
            t.done = true;
        }
        assert_eq!(completion_rate(&tasks), 1.0);
    }

    #[test]
    fn test_completion_rate_empty() {
        let tasks: Vec<Task> = vec![];
        assert_eq!(completion_rate(&tasks), 0.0);
    }
}
