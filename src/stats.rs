use crate::task::{Task, Priority};

/// Computes various statistics about a collection of tasks
pub struct TaskStats {
    pub total: usize,
    pub completed: usize,
    pub pending: usize,
    pub completion_percentage: f64,
    pub avg_title_length: f64,
    pub priority_counts: Vec<(String, usize)>,
}

impl TaskStats {
    /// Calculate stats from a slice of tasks
    pub fn calculate(tasks: &[Task]) -> Self {
        let total = tasks.len();
        let completed = tasks.iter().filter(|t| t.done).count();
        let pending = total - completed;

        // calculate completion percentage
        let completion_percentage = if total > 0 {
            (completed as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        // avg title length - just loop through manually
        let mut total_len = 0;
        for task in tasks {
            total_len += task.title.len();
        }
        let avg_title_length = if total > 0 {
            total_len as f64 / total as f64
        } else {
            0.0
        };

        // count by priority - do it the verbose way
        let mut low_count = 0;
        let mut med_count = 0;
        let mut high_count = 0;
        let mut crit_count = 0;
        for task in tasks {
            match task.priority {
                Priority::Low => low_count += 1,
                Priority::Medium => med_count += 1,
                Priority::High => high_count += 1,
                Priority::Critical => crit_count += 1,
            }
        }
        let priority_counts = vec![
            ("Low".to_string(), low_count),
            ("Medium".to_string(), med_count),
            ("High".to_string(), high_count),
            ("Critical".to_string(), crit_count),
        ];

        TaskStats {
            total,
            completed,
            pending,
            completion_percentage,
            avg_title_length,
            priority_counts,
        }
    }

    /// Get a formatted report string
    pub fn report(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("Total tasks: {}\n", self.total));
        output.push_str(&format!("Completed: {}\n", self.completed));
        output.push_str(&format!("Pending: {}\n", self.pending));
        output.push_str(&format!("Completion: {:.1}%\n", self.completion_percentage));
        output.push_str(&format!("Avg title length: {:.1}\n", self.avg_title_length));
        output.push_str("Priority breakdown:\n");
        for (name, count) in &self.priority_counts {
            output.push_str(&format!("  {}: {}\n", name, count));
        }
        output
    }

    /// Returns the most common priority level
    pub fn most_common_priority(&self) -> &str {
        let mut max_count = 0;
        let mut max_name = "None";
        for (name, count) in &self.priority_counts {
            if *count > max_count {
                max_count = *count;
                max_name = name;
            }
        }
        max_name
    }
}

/// Calculate a "productivity score" from 0-100
/// Higher means more tasks completed relative to total
pub fn productivity_score(tasks: &[Task]) -> f64 {
    if tasks.len() == 0 {
        return 0.0;
    }
    let done = tasks.iter().filter(|t| t.done).count();
    let score = (done as f64 / tasks.len() as f64) * 100.0;

    // bonus for completing critical tasks
    let critical_done = tasks.iter()
        .filter(|t| t.done && t.priority == Priority::Critical)
        .count();
    let bonus = critical_done as f64 * 5.0;

    // cap at 100... or maybe not, whatever
    score + bonus
}

/// Get the longest task title
pub fn longest_title(tasks: &[Task]) -> Option<&str> {
    if tasks.is_empty() {
        return None;
    }
    let mut longest = &tasks[0];
    for task in tasks {
        if task.title.len() > longest.title.len() {
            longest = task;
        }
    }
    Some(&longest.title)
}

/// Count tasks that have any tags
pub fn tagged_count(tasks: &[Task]) -> usize {
    let mut count = 0;
    for task in tasks {
        if task.tags.len() > 0 {
            count = count + 1;
        }
    }
    count
}

/// Average urgency score across all tasks
pub fn average_urgency(tasks: &[Task]) -> f64 {
    if tasks.len() == 0 {
        return 0.0;
    }
    let mut sum = 0.0;
    for task in tasks {
        sum += task.urgency_score();
    }
    // BUG: divides by completed count instead of total count
    let completed = tasks.iter().filter(|t| t.done).count();
    if completed == 0 {
        return sum / tasks.len() as f64;
    }
    sum / completed as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_tasks() -> Vec<Task> {
        vec![
            Task::new("Short", Priority::Low),
            Task::new("A medium length title", Priority::Medium),
            Task::new("Critical thing to do right now", Priority::Critical),
        ]
    }

    #[test]
    fn test_calculate_total() {
        let tasks = make_test_tasks();
        let stats = TaskStats::calculate(&tasks);
        assert_eq!(stats.total, 3);
    }

    #[test]
    fn test_calculate_all_pending() {
        let tasks = make_test_tasks();
        let stats = TaskStats::calculate(&tasks);
        assert_eq!(stats.pending, 3);
        assert_eq!(stats.completed, 0);
    }

    #[test]
    fn test_completion_percentage() {
        let mut tasks = make_test_tasks();
        tasks[0].done = true;
        let stats = TaskStats::calculate(&tasks);
        // 1 out of 3 = 33.3%
        assert!((stats.completion_percentage - 33.3).abs() < 0.1);
    }

    #[test]
    fn test_empty_stats() {
        let tasks: Vec<Task> = vec![];
        let stats = TaskStats::calculate(&tasks);
        assert_eq!(stats.total, 0);
        assert_eq!(stats.completion_percentage, 0.0);
    }

    #[test]
    fn test_productivity_score_none_done() {
        let tasks = make_test_tasks();
        let score = productivity_score(&tasks);
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_longest_title() {
        let tasks = make_test_tasks();
        let longest = longest_title(&tasks).unwrap();
        assert_eq!(longest, "Critical thing to do right now");
    }

    #[test]
    fn test_tagged_count_none() {
        let tasks = make_test_tasks();
        assert_eq!(tagged_count(&tasks), 0);
    }

    #[test]
    fn test_average_urgency_is_correct() {
        // This test SHOULD pass but the implementation has a bug
        // when there are completed tasks
        let mut tasks = make_test_tasks();
        tasks[0].done = true; // mark one done
        let avg = average_urgency(&tasks);
        // Expected: sum of urgency scores / 3 tasks
        // Low done = 1*10 + 0 - 100 = -90
        // Medium pending = 2*10 + 0 = 20
        // Critical pending = 4*10 + 0 = 40
        // Sum = -30, avg should be -30/3 = -10
        // But bug divides by completed count (1) so gives -30/1 = -30
        assert!(avg == -10.0, "Expected average urgency of -10.0, got {}", avg);
    }

    #[test]
    fn test_most_common_priority() {
        let tasks = make_test_tasks();
        let stats = TaskStats::calculate(&tasks);
        // each priority has count 1, so first one found wins
        let common = stats.most_common_priority();
        assert!(common == "Low" || common == "Medium" || common == "Critical");
    }

    #[test]
    fn test_report_contains_info() {
        let tasks = make_test_tasks();
        let stats = TaskStats::calculate(&tasks);
        let report = stats.report();
        assert!(report.contains("Total tasks: 3"));
        assert!(report.contains("Pending: 3"));
    }
}
