use std::io;
use crate::storage::{load_data, save_data};
use crate::commands::user::get_current_user;

pub fn delete_task(id: u64) -> Result<(), io::Error> {
    let mut users = load_data();
    let username = get_current_user();
    
    if username.is_empty() {
        eprintln!("❌ Error: No user selected. Please create or switch to a user first");
        std::process::exit(1);
    }
    
    if let Some(user) = users.get_mut(&username) {
        if user.delete_task(id) {
            save_data(&users)?;
            println!("✅ Task {} deleted successfully for user '{}'", id, username);
            Ok(())
        } else {
            eprintln!("❌ Error: Task with ID {} not found for user '{}'", id, username);
            std::process::exit(1);
        }
    } else {
        eprintln!("❌ Error: User '{}' not found", username);
        std::process::exit(1);
    }
}