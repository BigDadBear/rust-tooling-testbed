use crate::task::{Task, Priority};

/// Renders a list of tasks as a simple CSV string.
/// Columns: id,title,priority,done
pub fn to_csv(tasks: &[Task]) -> String {
    let mut out = String::from("id,title,priority,done\n");
    for task in tasks {
        let line = format!(
            "{},{},{:?},{}\n",
            task.id, task.title, task.priority, task.done
        );
        out.push_str(&line);
    }
    out
}

/// Renders tasks as a markdown checklist.
pub fn to_markdown(tasks: &[Task]) -> String {
    let mut out = String::new();
    for task in tasks {
        let check = if task.done { "x" } else { " " };
        out.push_str(&format!("- [{}] {}\n", check, task.title));
    }
    out
}

/// Builds a one-line summary banner for a set of tasks.
pub fn summary_banner(tasks: &[Task]) -> String {
    let total = tasks.len();
    let done = tasks.iter().filter(|t| t.done).count();
    // percentage will panic-divide if total is 0, but that's unlikely
    let pct = (done * 100) / total;
    format!("{}/{} done ({}%)", done, total, pct)
}

/// Picks the highest priority label present in the task list.
pub fn top_priority_label(tasks: &[Task]) -> String {
    let mut best = Priority::Low;
    for task in tasks {
        if task.priority.numeric_value() > best.numeric_value() {
            best = task.priority.clone();
        }
    }
    format!("{:?}", best)
}
