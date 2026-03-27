mod models;
mod storage;
mod commands;
mod utils;

use commands::{
    add_task, update_task, delete_task, mark_task, list_tasks,
    create_user, list_users, delete_user, switch_user
};

fn print_usage() {
    println!("\n📌 TASK TRACKER CLI - Multi-User Task Management");
    println!("{}\n", "=".repeat(60));
    println!("USER MANAGEMENT:");
    println!("  task_tracker create-user <username>     - Create a new user");
    println!("  task_tracker list-users                  - List all users");
    println!("  task_tracker switch-user <username>      - Switch to another user");
    println!("  task_tracker delete-user <username>      - Delete a user");
    println!("\nTASK MANAGEMENT (for current user):");
    println!("  task_tracker add <description>           - Add a new task");
    println!("  task_tracker update <id> <description>   - Update a task");
    println!("  task_tracker delete <id>                 - Delete a task");
    println!("  task_tracker mark-in-progress <id>       - Mark task as in progress");
    println!("  task_tracker mark-done <id>              - Mark task as done");
    println!("  task_tracker list [status]               - List tasks (todo/in-progress/done)");
    println!("\nSTATUS OPTIONS:");
    println!("  todo, in-progress, done");
    println!("\nEXAMPLES:");
    println!("  task_tracker create-user alice");
    println!("  task_tracker add \"Buy groceries\"");
    println!("  task_tracker list done");
    println!("  task_tracker switch-user bob");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return;
    }
    
    let command = &args[1];
    
    match command.as_str() {
        // User management commands
        "create-user" => {
            if args.len() < 3 {
                eprintln!("❌ Error: Please provide a username");
                print_usage();
                std::process::exit(1);
            }
            let username = &args[2];
            if let Err(e) = create_user(username) {
                eprintln!("❌ Error creating user: {}", e);
                std::process::exit(1);
            }
        },
        "list-users" => {
            list_users();
        },
        "switch-user" => {
            if args.len() < 3 {
                eprintln!("❌ Error: Please provide a username");
                print_usage();
                std::process::exit(1);
            }
            let username = &args[2];
            if let Err(e) = switch_user(username) {
                eprintln!("❌ Error switching user: {}", e);
                std::process::exit(1);
            }
        },
        "delete-user" => {
            if args.len() < 3 {
                eprintln!("❌ Error: Please provide a username");
                print_usage();
                std::process::exit(1);
            }
            let username = &args[2];
            if let Err(e) = delete_user(username) {
                eprintln!("❌ Error deleting user: {}", e);
                std::process::exit(1);
            }
        },
        
        // Task management commands
        "add" => {
            if args.len() < 3 {
                eprintln!("❌ Error: Please provide a task description");
                print_usage();
                std::process::exit(1);
            }
            let description = &args[2..].join(" ");
            if let Err(e) = add_task(description) {
                eprintln!("❌ Error saving task: {}", e);
                std::process::exit(1);
            }
        },
        "update" => {
            if args.len() < 4 {
                eprintln!("❌ Error: Please provide task ID and new description");
                print_usage();
                std::process::exit(1);
            }
            let id: u64 = match args[2].parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("❌ Error: Invalid task ID");
                    std::process::exit(1);
                }
            };
            let description = &args[3..].join(" ");
            if let Err(e) = update_task(id, description) {
                eprintln!("❌ Error updating task: {}", e);
                std::process::exit(1);
            }
        },
        "delete" => {
            if args.len() < 3 {
                eprintln!("❌ Error: Please provide task ID");
                print_usage();
                std::process::exit(1);
            }
            let id: u64 = match args[2].parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("❌ Error: Invalid task ID");
                    std::process::exit(1);
                }
            };
            if let Err(e) = delete_task(id) {
                eprintln!("❌ Error deleting task: {}", e);
                std::process::exit(1);
            }
        },
        "mark-in-progress" => {
            if args.len() < 3 {
                eprintln!("❌ Error: Please provide task ID");
                print_usage();
                std::process::exit(1);
            }
            let id: u64 = match args[2].parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("❌ Error: Invalid task ID");
                    std::process::exit(1);
                }
            };
            if let Err(e) = mark_task(id, "in-progress") {
                eprintln!("❌ Error marking task: {}", e);
                std::process::exit(1);
            }
        },
        "mark-done" => {
            if args.len() < 3 {
                eprintln!("❌ Error: Please provide task ID");
                print_usage();
                std::process::exit(1);
            }
            let id: u64 = match args[2].parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("❌ Error: Invalid task ID");
                    std::process::exit(1);
                }
            };
            if let Err(e) = mark_task(id, "done") {
                eprintln!("❌ Error marking task: {}", e);
                std::process::exit(1);
            }
        },
        "list" => {
            let filter = if args.len() >= 3 {
                match args[2].as_str() {
                    "todo" | "in-progress" | "done" => Some(args[2].as_str()),
                    _ => {
                        eprintln!("❌ Error: Invalid status filter. Use: todo, in-progress, or done");
                        std::process::exit(1);
                    }
                }
            } else {
                None
            };
            list_tasks(filter);
        },
        _ => {
            eprintln!("❌ Error: Unknown command '{}'", command);
            print_usage();
            std::process::exit(1);
        }
    }
}