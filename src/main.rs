mod task;
mod store;
mod filters;
mod archive;

use store::TaskStore;
use task::{Task, Priority};
use archive::Archive;

fn main() {
    let mut store = TaskStore::new();

    store.add_task(Task::new("Buy groceries", Priority::Low));
    store.add_task(Task::new("Fix production bug", Priority::Critical));
    store.add_task(Task::new("Write docs", Priority::Medium));
    store.add_task(Task::new("Refactor auth module", Priority::High));

    println!("All tasks:");
    for task in store.get_all_tasks() {
        println!("  [{}] {} (priority: {:?})", 
            if task.done { "x" } else { " " },
            task.title, task.priority);
    }

    // mark first task done
    let id = store.get_all_tasks()[0].id;
    store.mark_done(id);

    println!("\nPending tasks: {}", store.pending_count());
    println!("Completed tasks: {}", store.done_count());

    // export to json
    let json = store.export_json();
    println!("\nExported JSON ({} bytes)", json.len());

    // archive whatever is done now
    let mut archive = Archive::new();
    let done_tasks = store.drain_done();
    for t in done_tasks {
        archive.archive_done(&mut vec![t]);
    }
    println!("\n{}", archive.report());
}
