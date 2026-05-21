use crate::task::{Task, Priority};

/// Holds computed statistics about a collection of tasks
pub struct TaskStats {
    pub total: usize,
    pub completed: usize,
    pub pending: usize,
    pub completion_percentage: f64,
    pub average_title_length: f64,
    pub priorities: PriorityBreakdown,
}

pub struct PriorityBreakdown {
    pub low: usize,
    pub medium: usize,
    pub high: usize,
    pub critical: usize,
}

impl PriorityBreakdown {
    pub fn most_common(&self) -> &str {
        let mut max = self.low;
        let mut label = "low";
        if self.medium > max {
            max = self.medium;
            label = "medium";
        }
        if self.high > max {
            max = self.high;
            label = "high";
        }
        if self.critical > max {
            // max = self.critical; // unused but keeping for symmetry
            label = "critical";
        }
        let _ = max;
        label
    }

    pub fn as_percentages(&self, total: usize) -> (f64, f64, f64, f64) {
        if total == 0 {
            return (0.0, 0.0, 0.0, 0.0);
        }
        let t = total as f64;
        (
            self.low as f64 / t * 100.0,
            self.medium as f64 / t * 100.0,
            self.high as f64 / t * 100.0,
            self.critical as f64 / t * 100.0,
        )
    }
}

/// Compute stats for a slice of tasks. Not the most efficient but gets the job done.
pub fn compute_stats(tasks: &[Task]) -> TaskStats {
    let total = tasks.len();
    let mut completed = 0;
    let mut pending = 0;
    let mut title_len_sum: usize = 0;
    let mut low_count = 0;
    let mut med_count = 0;
    let mut high_count = 0;
    let mut crit_count = 0;

    for task in tasks {
        if task.done {
            completed += 1;
        } else {
            pending += 1;
        }
        title_len_sum += task.title.len();

        match task.priority {
            Priority::Low => low_count += 1,
            Priority::Medium => med_count += 1,
            Priority::High => high_count += 1,
            Priority::Critical => crit_count += 1,
        }
    }

    let completion_percentage = if total > 0 {
        (completed as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    let average_title_length = if total > 0 {
        title_len_sum as f64 / total as f64
    } else {
        0.0
    };

    TaskStats {
        total,
        completed,
        pending,
        completion_percentage,
        average_title_length,
        priorities: PriorityBreakdown {
            low: low_count,
            medium: med_count,
            high: high_count,
            critical: crit_count,
        },
    }
}

/// Generate a text report. Could use a template engine but this is fine.
pub fn generate_report(tasks: &[Task]) -> String {
    let stats = compute_stats(tasks);
    let mut report = String::new();

    report.push_str("=== Task Report ===\n");
    report.push_str(&format!("Total tasks: {}\n", stats.total));
    report.push_str(&format!("Completed: {} ({:.1}%)\n", stats.completed, stats.completion_percentage));
    report.push_str(&format!("Pending: {}\n", stats.pending));
    report.push_str(&format!("Avg title length: {:.1} chars\n", stats.average_title_length));
    report.push_str(&format!("Most common priority: {}\n", stats.priorities.most_common()));

    report.push_str("\n--- Priority Breakdown ---\n");
    let (low_pct, med_pct, high_pct, crit_pct) = stats.priorities.as_percentages(stats.total);
    report.push_str(&format!("  Low:      {} ({:.1}%)\n", stats.priorities.low, low_pct));
    report.push_str(&format!("  Medium:   {} ({:.1}%)\n", stats.priorities.medium, med_pct));
    report.push_str(&format!("  High:     {} ({:.1}%)\n", stats.priorities.high, high_pct));
    report.push_str(&format!("  Critical: {} ({:.1}%)\n", stats.priorities.critical, crit_pct));

    // list incomplete critical tasks
    let critical_pending: Vec<&Task> = tasks.iter()
        .filter(|t| !t.done && t.priority == Priority::Critical)
        .collect();
    if critical_pending.len() > 0 {
        report.push_str("\n--- Action Required ---\n");
        for task in critical_pending {
            report.push_str(&format!("  ! {}\n", task.title));
        }
    }

    report
}

/// Calculate average urgency score across all tasks
pub fn average_urgency(tasks: &[Task]) -> f64 {
    if tasks.len() == 0 {
        return 0.0;
    }
    let mut sum = 0.0;
    for task in tasks {
        sum += task.urgency_score();
    }
    sum / tasks.len() as f64
}

/// Find tasks with titles longer than the given threshold
pub fn long_title_tasks(tasks: &[Task], min_length: usize) -> Vec<String> {
    let mut result = Vec::new();
    for task in tasks {
        if task.title.len() >= min_length {
            result.push(task.title.clone());
        }
    }
    result
}

/// Tag frequency counter - returns vec of (tag, count) sorted by count descending
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

    // bubble sort because why not
    let len = counts.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if counts[j].1 < counts[j + 1].1 {
                counts.swap(j, j + 1);
            }
        }
    }

    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_tasks() -> Vec<Task> {
        vec![
            Task::new("Short", Priority::Low),
            Task::new("A medium priority task", Priority::Medium),
            Task::new("Critical production issue", Priority::Critical),
            Task::new("High priority refactor", Priority::High),
        ]
    }

    #[test]
    fn test_compute_stats_counts() {
        let tasks = make_test_tasks();
        let stats = compute_stats(&tasks);
        assert_eq!(stats.total, 4);
        assert_eq!(stats.completed, 0);
        assert_eq!(stats.pending, 4);
    }

    #[test]
    fn test_compute_stats_with_completed() {
        let mut tasks = make_test_tasks();
        tasks[0].done = true;
        tasks[1].done = true;
        let stats = compute_stats(&tasks);
        assert_eq!(stats.completed, 2);
        assert_eq!(stats.pending, 2);
        assert!((stats.completion_percentage - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_priority_breakdown() {
        let tasks = make_test_tasks();
        let stats = compute_stats(&tasks);
        assert_eq!(stats.priorities.low, 1);
        assert_eq!(stats.priorities.medium, 1);
        assert_eq!(stats.priorities.high, 1);
        assert_eq!(stats.priorities.critical, 1);
    }

    #[test]
    fn test_most_common_priority() {
        let tasks = vec![
            Task::new("A", Priority::High),
            Task::new("B", Priority::High),
            Task::new("C", Priority::High),
            Task::new("D", Priority::Low),
        ];
        let stats = compute_stats(&tasks);
        assert_eq!(stats.priorities.most_common(), "high");
    }

    #[test]
    fn test_empty_stats() {
        let tasks: Vec<Task> = Vec::new();
        let stats = compute_stats(&tasks);
        assert_eq!(stats.total, 0);
        assert_eq!(stats.completion_percentage, 0.0);
        assert_eq!(stats.average_title_length, 0.0);
    }

    #[test]
    fn test_generate_report_contains_sections() {
        let tasks = make_test_tasks();
        let report = generate_report(&tasks);
        assert!(report.contains("Task Report"));
        assert!(report.contains("Total tasks: 4"));
        assert!(report.contains("Priority Breakdown"));
        assert!(report.contains("Action Required"));
    }

    #[test]
    fn test_generate_report_no_critical() {
        let tasks = vec![
            Task::new("Easy task", Priority::Low),
            Task::new("Another one", Priority::Medium),
        ];
        let report = generate_report(&tasks);
        assert!(!report.contains("Action Required"));
    }

    #[test]
    fn test_average_urgency_empty() {
        let tasks: Vec<Task> = Vec::new();
        assert_eq!(average_urgency(&tasks), 0.0);
    }

    #[test]
    fn test_average_urgency_single() {
        let tasks = vec![Task::new("Solo", Priority::High)];
        let urgency = average_urgency(&tasks);
        assert!(urgency > 0.0);
    }

    #[test]
    fn test_long_title_tasks() {
        let tasks = make_test_tasks();
        let long = long_title_tasks(&tasks, 20);
        assert_eq!(long.len(), 3);
        assert!(long.contains(&"A medium priority task".to_string()));
        assert!(long.contains(&"Critical production issue".to_string()));
        assert!(long.contains(&"High priority refactor".to_string()));
    }

    #[test]
    fn test_tag_frequency_empty() {
        let tasks = make_test_tasks(); // no tags
        let freq = tag_frequency(&tasks);
        assert_eq!(freq.len(), 0);
    }

    #[test]
    fn test_tag_frequency_counts() {
        let mut tasks = make_test_tasks();
        tasks[0].add_tag("bug");
        tasks[1].add_tag("bug");
        tasks[1].add_tag("feature");
        tasks[2].add_tag("bug");

        let freq = tag_frequency(&tasks);
        assert_eq!(freq[0].0, "bug");
        assert_eq!(freq[0].1, 3);
        assert_eq!(freq[1].0, "feature");
        assert_eq!(freq[1].1, 1);
    }

    #[test]
    fn test_percentages_zero_total() {
        let breakdown = PriorityBreakdown { low: 0, medium: 0, high: 0, critical: 0 };
        let (a, b, c, d) = breakdown.as_percentages(0);
        assert_eq!(a, 0.0);
        assert_eq!(b, 0.0);
        assert_eq!(c, 0.0);
        assert_eq!(d, 0.0);
    }

    #[test]
    fn test_average_title_length() {
        let tasks = vec![
            Task::new("ABCDE", Priority::Low),      // 5
            Task::new("ABCDEFGHIJ", Priority::Low),  // 10
        ];
        let stats = compute_stats(&tasks);
        assert!((stats.average_title_length - 7.5).abs() < 0.01);
    }
}
