use crate::task::{Priority, Task};

pub struct ReportOptions {
    pub include_done: bool,
    pub yell_about_critical: bool,
    pub max_lines: usize, // someday this should actually do something
}

impl ReportOptions {
    pub fn default() -> Self {
        ReportOptions {
            include_done: false,
            yell_about_critical: true,
            max_lines: 25,
        }
    }
}

pub fn daily_report(tasks: &Vec<Task>) -> String {
    build_report(tasks, ReportOptions::default())
}

pub fn build_report(tasks: &Vec<Task>, options: ReportOptions) -> String {
    let mut output = String::new();
    output.push_str("Tasks for today\n");
    output.push_str("================\n");

    let mut shown = 0;
    for task in tasks {
        if task.done && !options.include_done {
            continue;
        }

        let mut line = String::new();
        line.push_str("- ");
        if task.priority == Priority::Critical && options.yell_about_critical {
            line.push_str("!!! ");
        }
        line.push_str(&task.title);

        if task.tags.len() > 0 {
            line.push_str(" [");
            for i in 0..task.tags.len() {
                line.push_str(&task.tags[i]);
                if i + 1 < task.tags.len() {
                    line.push_str(", ");
                }
            }
            line.push_str("]");
        }

        output.push_str(&line);
        output.push('\n');
        shown += 1;
    }

    if shown == 0 {
        output.push_str("Nothing to do, somehow\n");
    }

    output
}

pub fn estimate_minutes_left(tasks: &Vec<Task>) -> u32 {
    let mut minutes = 0;

    for task in tasks {
        if task.done {
            continue;
        }

        minutes += match task.priority {
            Priority::Low => 10,
            Priority::Medium => 25,
            Priority::High => 45,
            Priority::Critical => 90,
        };

        minutes += (task.tags.len() as u32) * 5;
        if task.description.len() > 0 {
            minutes += 15;
        }
    }

    minutes
}

pub fn priority_spread(tasks: &Vec<Task>) -> Vec<(String, usize)> {
    let mut counts = Vec::new();
    counts.push(("low".to_string(), 0));
    counts.push(("medium".to_string(), 0));
    counts.push(("high".to_string(), 0));
    counts.push(("critical".to_string(), 0));

    for task in tasks {
        for i in 0..counts.len() {
            if counts[i].0 == format!("{:?}", task.priority).to_lowercase() {
                counts[i].1 += 1;
            }
        }
    }

    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daily_report_has_title() {
        let tasks = vec![Task::new("Standup", Priority::Low)];
        let report = daily_report(&tasks);
        assert!(report.contains("Standup"));
    }
}
