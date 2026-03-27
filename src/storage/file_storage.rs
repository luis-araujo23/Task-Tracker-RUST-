use std::fs;
use std::io;
use std::path::Path;
use std::collections::HashMap;
use crate::models::User;

const JSON_FILE: &str = "task_tracker_data.json";

pub type DataStore = HashMap<String, User>;

pub fn load_data() -> DataStore {
    if !Path::new(JSON_FILE).exists() {
        return HashMap::new();
    }

    let data = fs::read_to_string(JSON_FILE).unwrap_or_else(|_| String::from("{}"));
    serde_json::from_str(&data).unwrap_or_else(|_| HashMap::new())
}

pub fn save_data(users: &DataStore) -> Result<(), io::Error> {
    let json = serde_json::to_string_pretty(users)?;
    fs::write(JSON_FILE, json)?;
    Ok(())
}