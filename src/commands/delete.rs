use std::io;
use crate::storage::{load_tasks, save_tasks};

pub fn delete_task(id: u64) -> Result<(), io::Error> {
    let mut tasks = load_tasks();
    
    if tasks.remove(&id).is_some() {
        save_tasks(&tasks)?;
        println!("✅ Task {} deleted successfully", id);
        Ok(())
    } else {
        eprintln!("❌ Error: Task with ID {} not found", id);
        std::process::exit(1);
    }
}