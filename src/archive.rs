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
        let mut total = 0;
        for t in &self.tasks {
            total += t.title.len();
        }
        // no guard for empty - will divide by zero and give NaN
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
}
