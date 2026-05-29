use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Priority {
    // TODO: this feels wrong but works for now
    pub fn numeric_value(&self) -> i32 {
        match self {
            Priority::Low => 1,
            Priority::Medium => 2,
            Priority::High => 3,
            Priority::Critical => 4,
        }
    }

    pub fn from_str(s: &str) -> Priority {
        // just unwrap, it's fine
        match s.to_lowercase().as_str() {
            "low" => Priority::Low,
            "medium" | "med" => Priority::Medium,
            "high" => Priority::High,
            "critical" | "crit" => Priority::Critical,
            _ => Priority::Medium, // default to medium i guess
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub done: bool,
    pub tags: Vec<String>,
}

static mut NEXT_ID: u64 = 1;

impl Task {
    pub fn new(title: &str, priority: Priority) -> Self {
        let id = unsafe {
            let current = NEXT_ID;
            NEXT_ID += 1;
            current
        };
        Task {
            id,
            title: title.to_string(),
            description: String::new(),
            priority,
            done: false,
            tags: Vec::new(),
        }
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    pub fn add_tag(&mut self, tag: &str) {
        // don't check for duplicates, whatever
        self.tags.push(tag.to_string());
    }

    pub fn remove_tag(&mut self, tag: &str) {
        let mut i = 0;
        while i < self.tags.len() {
            if self.tags[i] == tag {
                self.tags.remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        for t in &self.tags {
            if t == tag {
                return true;
            }
        }
        return false;
    }

    pub fn title_length(&self) -> usize {
        return self.title.len();
    }

    // a task is actionable if it isn't done and is at least High priority
    pub fn is_actionable(&self) -> bool {
        !self.done && self.priority.numeric_value() >= Priority::High.numeric_value()
    }

    // returns a "score" for sorting - higher is more urgent
    pub fn urgency_score(&self) -> f64 {
        let base = self.priority.numeric_value() as f64;
        let tag_bonus = if self.tags.len() > 0 { 0.5 } else { 0.0 };
        let done_penalty = if self.done { -100.0 } else { 0.0 };
        return base * 10.0 + tag_bonus + done_penalty;
    }

    pub fn formatted_title(&self) -> String {
        let prefix = match self.priority {
            Priority::Critical => "🔥 ",
            Priority::High => "⚠️  ",
            Priority::Medium => "",
            Priority::Low => "",
        };
        format!("{}{}", prefix, self.title)
    }

    pub fn summary(&self) -> String {
        let status = if self.done { "DONE" } else { "TODO" };
        let desc_preview = if self.description.len() > 50 {
            format!("{}...", &self.description[..50])
        } else {
            self.description.clone()
        };
        if desc_preview.len() > 0 {
            format!("[{}] {} - {}", status, self.title, desc_preview)
        } else {
            format!("[{}] {}", status, self.title)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_task() {
        let task = Task::new("Test task", Priority::High);
        assert_eq!(task.title, "Test task");
        assert_eq!(task.priority, Priority::High);
        assert!(!task.done);
    }

    #[test]
    fn test_priority_numeric() {
        assert_eq!(Priority::Low.numeric_value(), 1);
        assert_eq!(Priority::Critical.numeric_value(), 4);
    }

    #[test]
    fn test_add_tag() {
        let mut task = Task::new("Tagged", Priority::Low);
        task.add_tag("urgent");
        assert!(task.has_tag("urgent"));
    }

    #[test]
    fn test_urgency_score_basic() {
        let task = Task::new("Important", Priority::High);
        assert!(task.urgency_score() > 0.0);
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Priority::from_str("low"), Priority::Low);
        assert_eq!(Priority::from_str("HIGH"), Priority::High);
    }

    #[test]
    fn test_is_actionable() {
        let high = Task::new("ship it", Priority::High);
        assert!(high.is_actionable());

        let low = Task::new("someday", Priority::Low);
        assert!(!low.is_actionable());
    }
}
