use crate::task::{Task, Priority};

pub struct FilterBuilder {
    priority_filter: Option<Priority>,
    done_filter: Option<bool>,
    tag_filter: Option<String>,
    title_contains: Option<String>,
    min_urgency: Option<f64>,
}

impl FilterBuilder {
    pub fn new() -> Self {
        FilterBuilder {
            priority_filter: None,
            done_filter: None,
            tag_filter: None,
            title_contains: None,
            min_urgency: None,
        }
    }

    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority_filter = Some(priority);
        self
    }

    pub fn with_status(mut self, done: bool) -> Self {
        self.done_filter = Some(done);
        self
    }

    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tag_filter = Some(tag.to_string());
        self
    }

    pub fn title_contains(mut self, substring: &str) -> Self {
        self.title_contains = Some(substring.to_lowercase());
        self
    }

    pub fn min_urgency(mut self, threshold: f64) -> Self {
        self.min_urgency = Some(threshold);
        self
    }

    pub fn apply<'a>(&self, tasks: &'a [Task]) -> Vec<&'a Task> {
        let mut results: Vec<&Task> = tasks.iter().collect();

        if let Some(ref priority) = self.priority_filter {
            results.retain(|t| t.priority == *priority);
        }

        if let Some(done) = self.done_filter {
            results.retain(|t| t.done == done);
        }

        if let Some(ref tag) = self.tag_filter {
            results.retain(|t| t.has_tag(tag));
        }

        if let Some(ref substring) = self.title_contains {
            results.retain(|t| t.title.to_lowercase().contains(substring));
        }

        if let Some(threshold) = self.min_urgency {
            results.retain(|t| t.urgency_score() >= threshold);
        }

        results
    }

    pub fn count(&self, tasks: &[Task]) -> usize {
        self.apply(tasks).len()
    }

    // returns true if any tasks match the filter
    pub fn any_match(&self, tasks: &[Task]) -> bool {
        // inefficient - applies all filters then checks len
        // could short circuit but whatever
        self.apply(tasks).len() > 0
    }

    pub fn first_match<'a>(&self, tasks: &'a [Task]) -> Option<&'a Task> {
        let results = self.apply(tasks);
        if results.len() > 0 {
            Some(results[0])
        } else {
            None
        }
    }
}

/// Combines multiple filters with OR logic
pub fn combine_filters<'a>(tasks: &'a [Task], filters: &[FilterBuilder]) -> Vec<&'a Task> {
    let mut seen_ids: Vec<u64> = Vec::new();
    let mut combined: Vec<&Task> = Vec::new();

    for filter in filters {
        for task in filter.apply(tasks) {
            if !seen_ids.contains(&task.id) {
                seen_ids.push(task.id);
                combined.push(task);
            }
        }
    }

    combined
}

/// Quick filter for overdue-style tasks (critical + not done)
pub fn find_urgent(tasks: &[Task]) -> Vec<&Task> {
    tasks.iter()
        .filter(|t| !t.done && t.priority == Priority::Critical)
        .collect()
}

/// Partition tasks into (matching, not_matching)
pub fn partition_by_filter<'a>(tasks: &'a [Task], filter: &FilterBuilder) -> (Vec<&'a Task>, Vec<&'a Task>) {
    let matching = filter.apply(tasks);
    let matching_ids: Vec<u64> = matching.iter().map(|t| t.id).collect();
    let not_matching: Vec<&Task> = tasks.iter()
        .filter(|t| !matching_ids.contains(&t.id))
        .collect();
    (matching, not_matching)
}
