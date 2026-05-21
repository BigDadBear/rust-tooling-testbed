use crate::task::{Task, Priority};

#[derive(Debug, Clone, PartialEq)]
pub enum NotificationLevel {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub message: String,
    pub level: NotificationLevel,
    pub task_id: Option<u64>,
    pub read: bool,
    pub timestamp: u64,
}

impl Notification {
    pub fn new(message: &str, level: NotificationLevel) -> Self {
        Notification {
            message: message.to_string(),
            level,
            task_id: None,
            read: false,
            timestamp: 0, // whatever, we don't have a real clock
        }
    }

    pub fn for_task(message: &str, level: NotificationLevel, task_id: u64) -> Self {
        Notification {
            message: message.to_string(),
            level,
            task_id: Some(task_id),
            read: false,
            timestamp: 0,
        }
    }

    pub fn mark_read(&mut self) {
        self.read = true;
    }

    pub fn is_critical(&self) -> bool {
        self.level == NotificationLevel::Critical
    }

    pub fn display_text(&self) -> String {
        let prefix = match self.level {
            NotificationLevel::Info => "[INFO]",
            NotificationLevel::Warning => "[WARN]",
            NotificationLevel::Error => "[ERROR]",
            NotificationLevel::Critical => "[CRIT]",
        };
        format!("{} {}", prefix, self.message)
    }

    pub fn short_display(&self) -> String {
        if self.message.len() > 30 {
            format!("{}...", &self.message[..30])
        } else {
            self.message.clone()
        }
    }
}

pub struct NotificationQueue {
    notifications: Vec<Notification>,
    max_size: usize,
}

impl NotificationQueue {
    pub fn new() -> Self {
        NotificationQueue {
            notifications: Vec::new(),
            max_size: 100,
        }
    }

    pub fn with_capacity(max: usize) -> Self {
        NotificationQueue {
            notifications: Vec::new(),
            max_size: max,
        }
    }

    pub fn push(&mut self, notification: Notification) {
        if self.notifications.len() >= self.max_size {
            // remove oldest - not great perf but fine
            self.notifications.remove(0);
        }
        self.notifications.push(notification);
    }

    pub fn unread_count(&self) -> usize {
        let mut count = 0;
        for n in &self.notifications {
            if !n.read {
                count += 1;
            }
        }
        count
    }

    pub fn total_count(&self) -> usize {
        self.notifications.len()
    }

    pub fn get_unread(&self) -> Vec<&Notification> {
        let mut result = Vec::new();
        for n in &self.notifications {
            if !n.read {
                result.push(n);
            }
        }
        result
    }

    pub fn get_by_level(&self, level: NotificationLevel) -> Vec<&Notification> {
        let mut result = Vec::new();
        for n in &self.notifications {
            if n.level == level {
                result.push(n);
            }
        }
        result
    }

    pub fn mark_all_read(&mut self) {
        for n in &mut self.notifications {
            n.read = true;
        }
    }

    pub fn clear_read(&mut self) {
        self.notifications.retain(|n| !n.read);
    }

    pub fn get_for_task(&self, task_id: u64) -> Vec<&Notification> {
        let mut result = Vec::new();
        for n in &self.notifications {
            if n.task_id == Some(task_id) {
                result.push(n);
            }
        }
        result
    }

    pub fn has_critical(&self) -> bool {
        for n in &self.notifications {
            if n.level == NotificationLevel::Critical && !n.read {
                return true;
            }
        }
        false
    }

    pub fn summary(&self) -> String {
        let total = self.total_count();
        let unread = self.unread_count();
        let critical = self.get_by_level(NotificationLevel::Critical).len();
        let errors = self.get_by_level(NotificationLevel::Error).len();
        format!(
            "{} notifications ({} unread, {} critical, {} errors)",
            total, unread, critical, errors
        )
    }

    pub fn latest(&self) -> Option<&Notification> {
        self.notifications.last()
    }

    pub fn oldest_unread(&self) -> Option<&Notification> {
        for n in &self.notifications {
            if !n.read {
                return Some(n);
            }
        }
        None
    }

    pub fn dismiss(&mut self, index: usize) -> bool {
        if index < self.notifications.len() {
            self.notifications.remove(index);
            true
        } else {
            false
        }
    }

    pub fn bulk_dismiss_read(&mut self) -> usize {
        let before = self.notifications.len();
        self.notifications.retain(|n| !n.read);
        before - self.notifications.len()
    }
}

/// Generate notifications based on task state
pub fn generate_task_notifications(tasks: &[Task]) -> Vec<Notification> {
    let mut notifications = Vec::new();

    for task in tasks {
        if task.priority == Priority::Critical && !task.done {
            notifications.push(Notification::for_task(
                &format!("Critical task pending: {}", task.title),
                NotificationLevel::Critical,
                task.id,
            ));
        }

        if task.tags.len() > 5 {
            notifications.push(Notification::for_task(
                &format!("Task has too many tags: {}", task.title),
                NotificationLevel::Warning,
                task.id,
            ));
        }

        if task.title.len() > 100 {
            notifications.push(Notification::for_task(
                &format!("Task title is very long: {}...", &task.title[..50]),
                NotificationLevel::Info,
                task.id,
            ));
        }
    }

    let pending_count = tasks.iter().filter(|t| !t.done).count();
    if pending_count > 20 {
        notifications.push(Notification::new(
            &format!("You have {} pending tasks!", pending_count),
            NotificationLevel::Warning,
        ));
    }

    notifications
}

/// Format all notifications as a single string
pub fn format_notification_digest(queue: &NotificationQueue) -> String {
    let unread = queue.get_unread();
    if unread.is_empty() {
        return "No new notifications.".to_string();
    }

    let mut digest = format!("--- {} Unread Notifications ---\n", unread.len());
    for (i, n) in unread.iter().enumerate() {
        digest.push_str(&format!("{}. {}\n", i + 1, n.display_text()));
    }
    digest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_notification() {
        let n = Notification::new("hello", NotificationLevel::Info);
        assert_eq!(n.message, "hello");
        assert!(!n.read);
    }

    #[test]
    fn test_queue_push() {
        let mut q = NotificationQueue::new();
        q.push(Notification::new("test", NotificationLevel::Info));
        assert_eq!(q.total_count(), 1);
    }
}
