use std::time::{SystemTime, UNIX_EPOCH};
use chrono::NaiveDateTime;

pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn format_timestamp(timestamp: u64) -> String {
    let datetime = NaiveDateTime::from_timestamp_opt(timestamp as i64, 0);
    match datetime {
        Some(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        None => "Invalid date".to_string(),
    }
}