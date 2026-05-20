use std::time::{SystemTime, UNIX_EPOCH};
use crate::task::Task;

#[derive(Debug, Clone)]
pub enum HistoryEvent {
    Created { task_id: u64, title: String },
    Completed { task_id: u64 },
    Reopened { task_id: u64 },
    TagAdded { task_id: u64, tag: String },
    TagRemoved { task_id: u64, tag: String },
    PriorityChanged { task_id: u64, old: String, new: String },
    Deleted { task_id: u64, title: String },
}

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub timestamp: u64,
    pub event: HistoryEvent,
}

pub struct TaskHistory {
    entries: Vec<HistoryEntry>,
    max_entries: usize,
}

impl TaskHistory {
    pub fn new() -> Self {
        TaskHistory {
            entries: Vec::new(),
            max_entries: 10000,
        }
    }

    pub fn with_max_entries(max: usize) -> Self {
        TaskHistory {
            entries: Vec::new(),
            max_entries: max,
        }
    }

    fn now_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    pub fn record(&mut self, event: HistoryEvent) {
        if self.entries.len() >= self.max_entries {
            // just drop oldest entries, not great but works
            self.entries.drain(0..self.max_entries / 10);
        }
        self.entries.push(HistoryEntry {
            timestamp: Self::now_timestamp(),
            event,
        });
    }

    pub fn record_creation(&mut self, task: &Task) {
        self.record(HistoryEvent::Created {
            task_id: task.id,
            title: task.title.clone(),
        });
    }

    pub fn record_completion(&mut self, task_id: u64) {
        self.record(HistoryEvent::Completed { task_id });
    }

    pub fn record_reopen(&mut self, task_id: u64) {
        self.record(HistoryEvent::Reopened { task_id });
    }

    pub fn record_tag_added(&mut self, task_id: u64, tag: &str) {
        self.record(HistoryEvent::TagAdded {
            task_id,
            tag: tag.to_string(),
        });
    }

    pub fn record_tag_removed(&mut self, task_id: u64, tag: &str) {
        self.record(HistoryEvent::TagRemoved {
            task_id,
            tag: tag.to_string(),
        });
    }

    pub fn record_priority_change(&mut self, task_id: u64, old: &str, new: &str) {
        self.record(HistoryEvent::PriorityChanged {
            task_id,
            old: old.to_string(),
            new: new.to_string(),
        });
    }

    pub fn record_deletion(&mut self, task_id: u64, title: &str) {
        self.record(HistoryEvent::Deleted {
            task_id,
            title: title.to_string(),
        });
    }

    pub fn all_entries(&self) -> &[HistoryEntry] {
        &self.entries
    }

    pub fn entries_for_task(&self, task_id: u64) -> Vec<&HistoryEntry> {
        self.entries.iter()
            .filter(|e| self.event_task_id(&e.event) == Some(task_id))
            .collect()
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub fn entries_since(&self, timestamp: u64) -> Vec<&HistoryEntry> {
        self.entries.iter()
            .filter(|e| e.timestamp >= timestamp)
            .collect()
    }

    pub fn completions_count(&self) -> usize {
        self.entries.iter()
            .filter(|e| matches!(e.event, HistoryEvent::Completed { .. }))
            .count()
    }

    pub fn last_n_entries(&self, n: usize) -> &[HistoryEntry] {
        if n >= self.entries.len() {
            &self.entries
        } else {
            &self.entries[self.entries.len() - n..]
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn summary(&self) -> String {
        let total = self.entries.len();
        let creates = self.entries.iter()
            .filter(|e| matches!(e.event, HistoryEvent::Created { .. }))
            .count();
        let completes = self.completions_count();
        let deletes = self.entries.iter()
            .filter(|e| matches!(e.event, HistoryEvent::Deleted { .. }))
            .count();
        format!(
            "History: {} events ({} created, {} completed, {} deleted)",
            total, creates, completes, deletes
        )
    }

    fn event_task_id(&self, event: &HistoryEvent) -> Option<u64> {
        match event {
            HistoryEvent::Created { task_id, .. } => Some(*task_id),
            HistoryEvent::Completed { task_id } => Some(*task_id),
            HistoryEvent::Reopened { task_id } => Some(*task_id),
            HistoryEvent::TagAdded { task_id, .. } => Some(*task_id),
            HistoryEvent::TagRemoved { task_id, .. } => Some(*task_id),
            HistoryEvent::PriorityChanged { task_id, .. } => Some(*task_id),
            HistoryEvent::Deleted { task_id, .. } => Some(*task_id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::{Task, Priority};

    #[test]
    fn test_record_creation() {
        let mut history = TaskHistory::new();
        let task = Task::new("Test task", Priority::High);
        history.record_creation(&task);
        assert_eq!(history.entry_count(), 1);
    }

    #[test]
    fn test_record_completion() {
        let mut history = TaskHistory::new();
        history.record_completion(1);
        assert_eq!(history.completions_count(), 1);
    }

    #[test]
    fn test_entries_for_task() {
        let mut history = TaskHistory::new();
        let task = Task::new("Tracked", Priority::Medium);
        let id = task.id;
        history.record_creation(&task);
        history.record_completion(id);
        history.record_completion(999); // different task
        assert_eq!(history.entries_for_task(id).len(), 2);
    }

    #[test]
    fn test_clear() {
        let mut history = TaskHistory::new();
        history.record_completion(1);
        history.record_completion(2);
        history.clear();
        assert_eq!(history.entry_count(), 0);
    }

    // NOTE: no tests for:
    // - max_entries eviction behavior
    // - entries_since (timestamp dependent)
    // - last_n_entries edge cases
    // - record_reopen, record_tag_added, record_tag_removed
    // - record_priority_change, record_deletion
    // - summary output format
}
