use std::io;
use crate::storage::{load_tasks, save_tasks};
use crate::utils::current_timestamp;

pub fn mark_task(id: u64, status: &str) -> Result<(), io::Error> {
    let mut tasks = load_tasks();
    
    if let Some(task) = tasks.get_mut(&id) {
        task.update_status(status.to_string(), current_timestamp());
        save_tasks(&tasks)?;
        println!("✅ Task {} marked as {}", id, status);
        Ok(())
    } else {
        eprintln!("❌ Error: Task with ID {} not found", id);
        std::process::exit(1);
    }
}