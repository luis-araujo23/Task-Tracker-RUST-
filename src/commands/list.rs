use crate::storage::load_data;
use crate::commands::user::get_current_user;
use crate::utils::format_timestamp;

pub fn list_tasks(filter: Option<&str>) {
    let users = load_data();
    let username = get_current_user();
    
    if username.is_empty() {
        println!("📭 No user selected. Please create or switch to a user first.");
        println!("   Use 'task_tracker list-users' to see available users");
        return;
    }
    
    if let Some(user) = users.get(&username) {
        let tasks = user.list_tasks(filter);
        
        if tasks.is_empty() {
            println!("📭 No tasks found for user '{}'", username);
            if filter.is_some() {
                println!("💡 Try listing without filter or with a different status");
            }
            return;
        }
        
        let filter_text = match filter {
            Some(f) => format!("({})", f),
            None => "(all)".to_string(),
        };
        
        println!("\n📋 TASKS FOR USER: {} {}", username, filter_text);
        println!("{}\n", "=".repeat(60));
        
        for task in tasks {
            let status_icon = match task.status.as_str() {
                "done" => "✅",
                "in-progress" => "🔄",
                _ => "⭕",
            };
            
            println!("ID: {}", task.id);
            println!("  Description: {}", task.description);
            println!("  Status: {} {}", status_icon, task.status);
            println!("  Created: {}", format_timestamp(task.created_at));
            println!("  Updated: {}", format_timestamp(task.updated_at));
            println!("{}", "-".repeat(40));
        }
    } else {
        eprintln!("❌ Error: User '{}' not found", username);
        std::process::exit(1);
    }
}