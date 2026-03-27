use std::io;
use crate::models::Task;
use crate::storage::{load_tasks, save_tasks, get_next_id};
use crate::utils::current_timestamp;

pub fn add_task(description: &str) -> Result<(), io::Error> {
    let mut tasks = load_tasks();
    let id = get_next_id(&tasks);
    let timestamp = current_timestamp();
    let task = Task::new(id, description.to_string(), timestamp);
    tasks.insert(id, task);
    save_tasks(&tasks)?;
    println!("✅ Task added successfully (ID: {})", id);
    Ok(())
}