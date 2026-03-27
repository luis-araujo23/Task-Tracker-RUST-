use std::io;
use crate::storage::{load_data, save_data};
use crate::commands::user::get_current_user;
use crate::utils::current_timestamp;

pub fn add_task(description: &str) -> Result<(), io::Error> {
    let mut users = load_data();
    let username = get_current_user();
    
    if username.is_empty() {
        eprintln!("❌ Error: No user selected. Please create a user first:");
        eprintln!("   task_tracker create-user <username>");
        std::process::exit(1);
    }
    
    let timestamp = current_timestamp();
    
    if let Some(user) = users.get_mut(&username) {
        let task_id = user.add_task(description.to_string(), timestamp);
        save_data(&users)?;
        println!("✅ Task added successfully for user '{}' (ID: {})", username, task_id);
        Ok(())
    } else {
        eprintln!("❌ Error: User '{}' not found", username);
        std::process::exit(1);
    }
}