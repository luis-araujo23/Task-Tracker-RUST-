use std::io;
use std::fs;
use std::path::Path;
use crate::models::User;
use crate::storage::{load_data, save_data};
use crate::utils::current_timestamp;

const CURRENT_USER_FILE: &str = "task_tracker_current_user.txt";

pub fn get_current_user() -> String {
    let data = load_data();

    if Path::new(CURRENT_USER_FILE).exists() {
        if let Ok(saved_username) = fs::read_to_string(CURRENT_USER_FILE) {
            let username = saved_username.trim();
            if data.contains_key(username) {
                return username.to_string();
            }
        }
    }

    // Fallback deterministico para evitar elecciones aleatorias del HashMap.
    let mut usernames: Vec<String> = data.keys().cloned().collect();
    usernames.sort();
    usernames.into_iter().next().unwrap_or_default()
}

pub fn set_current_user(username: &str) {
    if username.is_empty() {
        let _ = fs::remove_file(CURRENT_USER_FILE);
        return;
    }

    let _ = fs::write(CURRENT_USER_FILE, username);
}

pub fn create_user(username: &str) -> Result<(), io::Error> {
    let mut users = load_data();
    
    if users.contains_key(username) {
        eprintln!("❌ Error: User '{}' already exists", username);
        std::process::exit(1);
    }
    
    let timestamp = current_timestamp();
    let user = User::new(username.to_string(), timestamp);
    users.insert(username.to_string(), user);
    save_data(&users)?;
    
    // Set as current user
    set_current_user(username);
    
    println!("✅ User '{}' created successfully and set as current user", username);
    Ok(())
}

pub fn list_users() {
    let users = load_data();
    
    if users.is_empty() {
        println!("📭 No users found.");
        return;
    }
    
    let current_user = get_current_user();
    
    println!("\n👥 USERS LIST:");
    println!("{}\n", "=".repeat(50));
    
    let mut users_vec: Vec<&User> = users.values().collect();
    users_vec.sort_by_key(|u| &u.username);
    
    for user in users_vec {
        let task_count = user.tasks.len();
        let marker = if user.username == current_user { "👉 " } else { "   " };
        println!("{}{} ({} tasks)", marker, user.username, task_count);
        println!("   Created: {}", crate::utils::format_timestamp(user.created_at));
        println!("   Last login: {}", crate::utils::format_timestamp(user.last_login));
        println!("{}", "-".repeat(40));
    }
}

pub fn delete_user(username: &str) -> Result<(), io::Error> {
    let mut users = load_data();
    
    if !users.contains_key(username) {
        eprintln!("❌ Error: User '{}' not found", username);
        std::process::exit(1);
    }
    
    let current_user = get_current_user();
    
    users.remove(username);
    save_data(&users)?;
    
    // If we deleted the current user, clear it
    if current_user == username {
        let mut usernames: Vec<&String> = users.keys().collect();
        usernames.sort();

        if let Some(first_user) = usernames.first() {
            set_current_user(first_user);
            println!("⚠️  Current user '{}' was deleted. Switched to '{}'", username, first_user);
        } else {
            set_current_user("");
            println!("⚠️  Current user '{}' was deleted. No users left.", username);
        }
    }
    
    println!("✅ User '{}' deleted successfully", username);
    Ok(())
}

pub fn switch_user(username: &str) -> Result<(), io::Error> {
    let users = load_data();
    
    if !users.contains_key(username) {
        eprintln!("❌ Error: User '{}' not found", username);
        eprintln!("💡 Use 'task_tracker list-users' to see available users");
        std::process::exit(1);
    }
    
    set_current_user(username);
    
    // Update last login time
    let mut users = load_data();
    if let Some(user) = users.get_mut(username) {
        user.update_last_login(crate::utils::current_timestamp());
        save_data(&users)?;
    }
    
    println!("✅ Switched to user '{}'", username);
    Ok(())
}