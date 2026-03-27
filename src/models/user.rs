use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use super::Task;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub username: String,
    pub tasks: HashMap<u64, Task>,
    pub next_id: u64,
    pub created_at: u64,
    pub last_login: u64,
}

impl User {
    pub fn new(username: String, timestamp: u64) -> Self {
        User {
            username,
            tasks: HashMap::new(),
            next_id: 1,
            created_at: timestamp,
            last_login: timestamp,
        }
    }
    
    pub fn add_task(&mut self, description: String, timestamp: u64) -> u64 {
        let id = self.next_id;
        let task = Task::new(id, description, timestamp);
        self.tasks.insert(id, task);
        self.next_id += 1;
        id
    }
    
    pub fn update_task(&mut self, id: u64, description: String, timestamp: u64) -> bool {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.update_description(description, timestamp);
            true
        } else {
            false
        }
    }
    
    pub fn delete_task(&mut self, id: u64) -> bool {
        self.tasks.remove(&id).is_some()
    }
    
    pub fn mark_task(&mut self, id: u64, status: String, timestamp: u64) -> bool {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.update_status(status, timestamp);
            true
        } else {
            false
        }
    }
    
    pub fn get_task(&self, id: u64) -> Option<&Task> {
        self.tasks.get(&id)
    }
    
    pub fn list_tasks(&self, filter: Option<&str>) -> Vec<&Task> {
        let mut tasks: Vec<&Task> = self.tasks.values().collect();
        tasks.sort_by_key(|t| t.id);
        
        match filter {
            Some("done") => tasks.into_iter().filter(|t| t.status == "done").collect(),
            Some("in-progress") => tasks.into_iter().filter(|t| t.status == "in-progress").collect(),
            Some("todo") => tasks.into_iter().filter(|t| t.status == "todo").collect(),
            _ => tasks,
        }
    }
    
    pub fn update_last_login(&mut self, timestamp: u64) {
        self.last_login = timestamp;
    }
}