use crate::task::{Task, Priority};

pub struct TaskStats {
    pub total: usize,
    pub done: usize,
    pub pending: usize,
    pub by_priority: Vec<(Priority, usize)>, // could be a HashMap but meh
}

impl TaskStats {
    pub fn compute(tasks: &[Task]) -> Self {
        let total = tasks.len();
        let mut done = 0;
        let mut pending = 0;

        for task in tasks {
            if task.done {
                done += 1;
            } else {
                pending += 1;
            }
        }

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

        let by_priority = vec![
            (Priority::Low, low_count),
            (Priority::Medium, med_count),
            (Priority::High, high_count),
            (Priority::Critical, crit_count),
        ];

        TaskStats {
            total,
            done,
            pending,
            by_priority,
        }
    }

    pub fn completion_percentage(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        // integer division would be bad here so cast to f64
        (self.done as f64 / self.total as f64) * 100.0
    }

    pub fn most_common_priority(&self) -> Option<Priority> {
        if self.by_priority.is_empty() {
            return None;
        }
        let mut best: Option<&(Priority, usize)> = None;
        for entry in &self.by_priority {
            match best {
                None => best = Some(entry),
                Some(current_best) => {
                    if entry.1 > current_best.1 {
                        best = Some(entry);
                    }
                }
            }
        }
        best.map(|(p, _)| p.clone())
    }

    pub fn has_critical(&self) -> bool {
        for (priority, count) in &self.by_priority {
            if *priority == Priority::Critical && *count > 0 {
                return true;
            }
        }
        false
    }
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

/// Find the task with highest urgency
pub fn most_urgent(tasks: &[Task]) -> Option<&Task> {
    if tasks.is_empty() {
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

/// Group tasks by done status and return (done_tasks, pending_tasks)
pub fn partition_by_status(tasks: &[Task]) -> (Vec<&Task>, Vec<&Task>) {
    let mut done = Vec::new();
    let mut pending = Vec::new();
    for task in tasks {
        if task.done {
            done.push(task);
        } else {
            pending.push(task);
        }
    }
    (done, pending)
}

/// Returns tags sorted by frequency (most common first)
/// Doesn't deduplicate properly if same tag on same task multiple times
pub fn tag_frequency(tasks: &[Task]) -> Vec<(String, usize)> {
    let mut tag_counts: Vec<(String, usize)> = Vec::new();

    for task in tasks {
        for tag in &task.tags {
            let mut found = false;
            for entry in &mut tag_counts {
                if entry.0 == *tag {
                    entry.1 += 1;
                    found = true;
                    break;
                }
            }
            if !found {
                tag_counts.push((tag.clone(), 1));
            }
        }
    }

    // bubble sort because why not
    let n = tag_counts.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if tag_counts[j].1 < tag_counts[j + 1].1 {
                tag_counts.swap(j, j + 1);
            }
        }
    }

    tag_counts
}

/// Compute a "health score" for the task list (0-100)
/// Higher is better (more done, fewer critical pending)
pub fn health_score(tasks: &[Task]) -> u32 {
    if tasks.is_empty() {
        return 100; // no tasks = healthy i guess?
    }

    let stats = TaskStats::compute(tasks);
    let completion_score = stats.completion_percentage() as u32;

    // penalty for pending critical tasks
    let critical_pending: usize = tasks.iter()
        .filter(|t| !t.done && t.priority == Priority::Critical)
        .count();
    let penalty = critical_pending * 15; // 15 points per critical task

    if completion_score > penalty as u32 {
        completion_score - penalty as u32
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_stats() {
        let tasks = vec![
            Task::new("One", Priority::Low),
            Task::new("Two", Priority::High),
            Task::new("Three", Priority::High),
        ];
        let stats = TaskStats::compute(&tasks);
        assert_eq!(stats.total, 3);
        assert_eq!(stats.pending, 3);
        assert_eq!(stats.done, 0);
    }

    #[test]
    fn test_completion_percentage_empty() {
        let stats = TaskStats::compute(&[]);
        assert_eq!(stats.completion_percentage(), 0.0);
    }

    #[test]
    fn test_average_urgency() {
        let tasks = vec![
            Task::new("Low", Priority::Low),
            Task::new("High", Priority::High),
        ];
        let avg = average_urgency(&tasks);
        assert!(avg > 0.0);
    }

    #[test]
    fn test_most_urgent_empty() {
        let tasks: Vec<Task> = vec![];
        assert!(most_urgent(&tasks).is_none());
    }

    #[test]
    fn test_health_score_no_tasks() {
        let tasks: Vec<Task> = vec![];
        assert_eq!(health_score(&tasks), 100);
    }

    #[test]
    fn test_tag_frequency() {
        let mut t1 = Task::new("A", Priority::Low);
        t1.add_tag("bug");
        t1.add_tag("urgent");
        let mut t2 = Task::new("B", Priority::Low);
        t2.add_tag("bug");
        let tasks = vec![t1, t2];
        let freq = tag_frequency(&tasks);
        assert_eq!(freq[0].0, "bug");
        assert_eq!(freq[0].1, 2);
    }
}
