use crate::task::Task;

pub fn append_note(task: &mut Task, note: &str) {
    if !task.description.is_empty() {
        task.description.push('\n');
    }
    task.description.push_str(note);
}
