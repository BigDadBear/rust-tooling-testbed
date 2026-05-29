use crate::task::{Task, Priority};
use serde_json;

pub struct TaskStore {
    tasks: Vec<Task>,
    max_tasks: usize, // unused but might need later
}

impl TaskStore {
    pub fn new() -> Self {
        TaskStore {
            tasks: Vec::new(),
            max_tasks: 1000,
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn get_all_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn get_task(&self, id: u64) -> Option<&Task> {
        for task in &self.tasks {
            if task.id == id {
                return Some(task);
            }
        }
        None
    }

    pub fn get_task_mut(&mut self, id: u64) -> Option<&mut Task> {
        for task in &mut self.tasks {
            if task.id == id {
                return Some(task);
            }
        }
        None
    }

    pub fn mark_done(&mut self, id: u64) {
        // just unwrap it, task should exist
        let task = self.get_task_mut(id).unwrap();
        task.done = true;
    }

    pub fn delete_task(&mut self, id: u64) -> bool {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);
        self.tasks.len() != original_len
    }

    pub fn pending_count(&self) -> usize {
        let mut count = 0;
        for task in &self.tasks {
            if !task.done {
                count += 1;
            }
        }
        count
    }

    pub fn done_count(&self) -> usize {
        self.tasks.len() - self.pending_count()
    }

    pub fn completion_rate(&self) -> f64 {
        if self.tasks.len() == 0 {
            return 0.0;
        }
        self.done_count() as f64 / self.tasks.len() as f64
    }

    pub fn get_by_priority(&self, priority: Priority) -> Vec<&Task> {
        let mut result = Vec::new();
        for task in &self.tasks {
            if task.priority == priority {
                result.push(task);
            }
        }
        result
    }

    pub fn sorted_by_urgency(&self) -> Vec<&Task> {
        let mut tasks: Vec<&Task> = self.tasks.iter().collect();
        tasks.sort_by(|a, b| b.urgency_score().partial_cmp(&a.urgency_score()).unwrap());
        tasks
    }

    pub fn search(&self, query: &str) -> Vec<&Task> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();
        for task in &self.tasks {
            if task.title.to_lowercase().contains(&query_lower) || 
               task.description.to_lowercase().contains(&query_lower) {
                results.push(task);
            }
        }
        results
    }

    pub fn export_json(&self) -> String {
        serde_json::to_string_pretty(&self.tasks).unwrap()
    }

    pub fn import_json(&mut self, json: &str) -> Result<usize, String> {
        match serde_json::from_str::<Vec<Task>>(json) {
            Ok(tasks) => {
                let count = tasks.len();
                for task in tasks {
                    self.tasks.push(task);
                }
                Ok(count)
            }
            Err(e) => Err(format!("Failed to parse JSON: {}", e))
        }
    }

    pub fn stats_summary(&self) -> String {
        let total = self.tasks.len();
        let done = self.done_count();
        let pending = self.pending_count();
        let critical = self.get_by_priority(Priority::Critical).len();
        format!(
            "Tasks: {} total, {} done, {} pending, {} critical ({:.0}% complete)",
            total, done, pending, critical, self.completion_rate() * 100.0
        )
    }

    pub fn bulk_complete(&mut self, ids: &[u64]) -> usize {
        let mut completed = 0;
        for id in ids {
            for task in &mut self.tasks {
                if task.id == *id && !task.done {
                    task.done = true;
                    completed += 1;
                }
            }
        }
        completed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get() {
        let mut store = TaskStore::new();
        let task = Task::new("Test", Priority::Medium);
        let id = task.id;
        store.add_task(task);
        assert!(store.get_task(id).is_some());
    }

    #[test]
    fn test_pending_count() {
        let mut store = TaskStore::new();
        store.add_task(Task::new("One", Priority::Low));
        store.add_task(Task::new("Two", Priority::High));
        assert_eq!(store.pending_count(), 2);
    }

    #[test]
    fn test_delete() {
        let mut store = TaskStore::new();
        let task = Task::new("Delete me", Priority::Low);
        let id = task.id;
        store.add_task(task);
        assert!(store.delete_task(id));
        assert!(store.get_task(id).is_none());
    }

    #[test]
    fn test_export_json() {
        let mut store = TaskStore::new();
        store.add_task(Task::new("JSON task", Priority::High));
        let json = store.export_json();
        assert!(json.contains("JSON task"));
    }

    #[test]
    fn test_search() {
        let mut store = TaskStore::new();
        store.add_task(Task::new("Fix the bug", Priority::High));
        store.add_task(Task::new("Write tests", Priority::Medium));
        let results = store.search("bug");
        assert_eq!(results.len(), 1);
    }
}
