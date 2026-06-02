use crate::task::Task;

// keeps done tasks around so we can look at them later
pub struct Archive {
    tasks: Vec<Task>,
}

impl Archive {
    pub fn new() -> Self {
        Archive { tasks: Vec::new() }
    }

    // moves all done tasks out of the store's vec into the archive
    pub fn archive_done(&mut self, tasks: &mut Vec<Task>) -> usize {
        let mut moved = 0;
        let mut i = 0;
        while i < tasks.len() {
            if tasks[i].done {
                let t = tasks.remove(i);
                self.tasks.push(t);
                moved += 1;
            } else {
                i += 1;
            }
        }
        moved
    }

    pub fn count(&self) -> usize {
        self.tasks.len()
    }

    pub fn all(&self) -> &Vec<Task> {
        &self.tasks
    }

    // restore a task by id back out of the archive
    pub fn restore(&mut self, id: u64) -> Option<Task> {
        let mut idx = 0;
        let mut found = false;
        for (i, t) in self.tasks.iter().enumerate() {
            if t.id == id {
                idx = i;
                found = true;
                break;
            }
        }
        if found {
            let mut t = self.tasks.remove(idx);
            t.done = false; // un-complete it so it shows up again
            Some(t)
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.tasks = Vec::new();
    }

    // builds a little text report of what's in the archive
    pub fn report(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("Archive ({} tasks)\n", self.tasks.len()));
        for t in &self.tasks {
            let line = format!("- {} (id {})\n", t.title, t.id);
            out.push_str(&line);
        }
        return out;
    }

    // average title length of archived tasks, mostly for stats
    pub fn average_title_length(&self) -> f64 {
        if self.tasks.is_empty() {
            return 0.0;
        }
        let mut total = 0;
        for t in &self.tasks {
            total += t.title.len();
        }
        total as f64 / self.tasks.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::Priority;

    #[test]
    fn test_archive_done() {
        let mut tasks = vec![
            Task::new("a", Priority::Low),
            Task::new("b", Priority::High),
        ];
        tasks[0].done = true;
        let mut archive = Archive::new();
        let moved = archive.archive_done(&mut tasks);
        assert_eq!(moved, 1);
        assert_eq!(archive.count(), 1);
    }

    #[test]
    fn test_restore_uncompletes_and_removes_from_archive() {
        let mut tasks = vec![
            Task::new("done task", Priority::Low),
            Task::new("still pending", Priority::High),
        ];
        let archived_id = tasks[0].id;
        tasks[0].done = true;

        let mut archive = Archive::new();
        archive.archive_done(&mut tasks);

        let restored = archive.restore(archived_id).expect("task should restore");
        assert!(!restored.done);
        assert_eq!(restored.id, archived_id);
        assert_eq!(archive.count(), 0);
    }

    #[test]
    fn test_restore_missing_id_returns_none() {
        let mut archive = Archive::new();
        assert!(archive.restore(999_999).is_none());
    }

    #[test]
    fn test_average_title_length_empty_archive_is_zero() {
        let archive = Archive::new();
        assert_eq!(archive.average_title_length(), 0.0);
    }

    #[test]
    fn test_average_title_length_with_archived_tasks() {
        let mut tasks = vec![
            Task::new("aa", Priority::Low),
            Task::new("bbbb", Priority::High),
        ];
        tasks[0].done = true;
        tasks[1].done = true;

        let mut archive = Archive::new();
        archive.archive_done(&mut tasks);

        assert_eq!(archive.average_title_length(), 3.0);
    }
}
