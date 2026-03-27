use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub description: String,
    pub status: String,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Task {
    pub fn new(id: u64, description: String, timestamp: u64) -> Self {
        Task {
            id,
            description,
            status: "todo".to_string(),
            created_at: timestamp,
            updated_at: timestamp,
        }
    }
    
    pub fn update_description(&mut self, description: String, timestamp: u64) {
        self.description = description;
        self.updated_at = timestamp;
    }
    
    pub fn update_status(&mut self, status: String, timestamp: u64) {
        self.status = status;
        self.updated_at = timestamp;
    }
}