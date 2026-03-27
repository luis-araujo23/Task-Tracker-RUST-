use std::fs;
use std::io;
use std::path::Path;
use std::collections::HashMap;
use crate::models::Task;

const JSON_FILE: &str = "tasks.json";

pub fn load_tasks() -> HashMap<u64, Task> {
    if !Path::new(JSON_FILE).exists() {
        return HashMap::new();
    }

    let data = fs::read_to_string(JSON_FILE).unwrap_or_else(|_| String::from("{}"));
    serde_json::from_str(&data).unwrap_or_else(|_| HashMap::new())
}

pub fn save_tasks(tasks: &HashMap<u64, Task>) -> Result<(), io::Error> {
    let json = serde_json::to_string_pretty(tasks)?;
    fs::write(JSON_FILE, json)?;
    Ok(())
}

pub fn get_next_id(tasks: &HashMap<u64, Task>) -> u64 {
    tasks.keys().max().unwrap_or(&0) + 1
}