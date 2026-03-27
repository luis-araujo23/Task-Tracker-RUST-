use crate::storage::load_tasks;
use crate::utils::format_timestamp;

pub fn list_tasks(filter: Option<&str>) {
    let tasks = load_tasks();
    
    if tasks.is_empty() {
        println!("📭 No tasks found.");
        return;
    }
    
    let mut tasks_vec: Vec<&crate::models::Task> = tasks.values().collect();
    tasks_vec.sort_by_key(|t| t.id);
    
    println!("\n📋 TASKS LIST:");
    println!("{}\n", "=".repeat(80));
    
    for task in tasks_vec {
        match filter {
            Some("done") if task.status != "done" => continue,
            Some("todo") if task.status != "todo" => continue,
            Some("in-progress") if task.status != "in-progress" => continue,
            Some(_) => {},
            None => {},
        }
        
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
}