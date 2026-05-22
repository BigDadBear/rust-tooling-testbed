use crate::task::{Task, Priority};

pub enum OutputFormat {
    Markdown,
    Csv,
    Plain,
    Html,
}

pub struct TaskFormatter {
    pub format: OutputFormat,
    pub include_done: bool,
    pub show_ids: bool,
    _internal_buffer: String, // might use this for caching later
}

impl TaskFormatter {
    pub fn new(format: OutputFormat) -> Self {
        TaskFormatter {
            format,
            include_done: true,
            show_ids: false,
            _internal_buffer: String::new(),
        }
    }

    pub fn format_tasks(&self, tasks: &[Task]) -> String {
        let mut output = String::new();

        let filtered: Vec<&Task> = if self.include_done {
            tasks.iter().collect()
        } else {
            let mut v = Vec::new();
            for t in tasks {
                if !t.done {
                    v.push(t);
                }
            }
            v
        };

        match self.format {
            OutputFormat::Markdown => {
                output.push_str("# Task List\n\n");
                for task in &filtered {
                    let checkbox = if task.done { "[x]" } else { "[ ]" };
                    let priority_str = self.priority_to_string(&task.priority);
                    if self.show_ids {
                        output.push_str(&format!("- {} #{} {} ({})\n", checkbox, task.id, task.title, priority_str));
                    } else {
                        output.push_str(&format!("- {} {} ({})\n", checkbox, task.title, priority_str));
                    }
                }
            }
            OutputFormat::Csv => {
                output.push_str("id,title,priority,done\n");
                for task in &filtered {
                    let done_str = if task.done { "true" } else { "false" };
                    let priority_str = self.priority_to_string(&task.priority);
                    // no escaping for commas in titles... probably fine
                    output.push_str(&format!("{},{},{},{}\n", task.id, task.title, priority_str, done_str));
                }
            }
            OutputFormat::Plain => {
                for task in &filtered {
                    let status = if task.done { "DONE" } else { "TODO" };
                    output.push_str(&format!("[{}] {}\n", status, task.title));
                }
            }
            OutputFormat::Html => {
                output.push_str("<ul>\n");
                for task in &filtered {
                    let class = if task.done { "done" } else { "pending" };
                    // not sanitizing title for HTML... yolo
                    output.push_str(&format!("  <li class=\"{}\">{}</li>\n", class, task.title));
                }
                output.push_str("</ul>\n");
            }
        }

        output
    }

    fn priority_to_string(&self, priority: &Priority) -> String {
        match priority {
            Priority::Low => String::from("low"),
            Priority::Medium => String::from("medium"),
            Priority::High => String::from("high"),
            Priority::Critical => String::from("critical"),
        }
    }

    pub fn format_single(&self, task: &Task) -> String {
        // just wrap in a slice, not the most efficient but works
        let tasks = vec![task.clone()];
        self.format_tasks(&tasks)
    }

    pub fn count_lines(&self, tasks: &[Task]) -> usize {
        let formatted = self.format_tasks(tasks);
        let mut count = 0;
        for c in formatted.chars() {
            if c == '\n' {
                count += 1;
            }
        }
        return count;
    }

    pub fn total_title_chars(&self, tasks: &[Task]) -> usize {
        let mut total = 0;
        for task in tasks {
            total += task.title.len();
        }
        total
    }

    // longest title in the list
    pub fn longest_title(&self, tasks: &[Task]) -> Option<String> {
        if tasks.len() == 0 {
            return None;
        }
        let mut longest = &tasks[0];
        for task in tasks {
            if task.title.len() > longest.title.len() {
                longest = task;
            }
        }
        Some(longest.title.clone())
    }
}

/// Renders a progress bar string like [=====>     ] 50%
pub fn render_progress_bar(done: usize, total: usize, width: usize) -> String {
    if total == 0 {
        return format!("[{}] 0%", " ".repeat(width));
    }

    let percentage = (done as f64 / total as f64 * 100.0) as usize;
    let filled = (done as f64 / total as f64 * width as f64) as usize;
    let empty = width - filled;

    let mut bar = String::from("[");
    for _ in 0..filled {
        bar.push('=');
    }
    if filled < width {
        bar.push('>');
        for _ in 0..(empty - 1) {
            bar.push(' ');
        }
    }
    bar.push(']');
    format!("{} {}%", bar, percentage)
}

/// Super basic table formatter - doesn't handle alignment well
pub fn format_as_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    let mut output = String::new();

    // header row
    let header_line = headers.join(" | ");
    output.push_str(&header_line);
    output.push('\n');

    // separator - just dashes, not aligned to columns
    let sep: String = "-".repeat(header_line.len());
    output.push_str(&sep);
    output.push('\n');

    // data rows
    for row in rows {
        let line = row.join(" | ");
        output.push_str(&line);
        output.push('\n');
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_format() {
        let tasks = vec![
            Task::new("First task", Priority::High),
            Task::new("Second task", Priority::Low),
        ];
        let formatter = TaskFormatter::new(OutputFormat::Markdown);
        let output = formatter.format_tasks(&tasks);
        assert!(output.contains("# Task List"));
        assert!(output.contains("First task"));
    }

    #[test]
    fn test_csv_format() {
        let tasks = vec![Task::new("My task", Priority::Medium)];
        let formatter = TaskFormatter::new(OutputFormat::Csv);
        let output = formatter.format_tasks(&tasks);
        assert!(output.contains("id,title,priority,done"));
        assert!(output.contains("My task"));
    }

    #[test]
    fn test_progress_bar() {
        let bar = render_progress_bar(5, 10, 20);
        assert!(bar.contains("50%"));
    }

    #[test]
    fn test_progress_bar_zero() {
        let bar = render_progress_bar(0, 0, 10);
        assert!(bar.contains("0%"));
    }

    #[test]
    fn test_count_lines() {
        let tasks = vec![
            Task::new("A", Priority::Low),
            Task::new("B", Priority::High),
        ];
        let formatter = TaskFormatter::new(OutputFormat::Plain);
        let lines = formatter.count_lines(&tasks);
        assert_eq!(lines, 2);
    }
}
