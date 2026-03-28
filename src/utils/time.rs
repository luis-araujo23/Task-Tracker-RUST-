use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{Local, TimeZone};

pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn format_timestamp(timestamp: u64) -> String {
    // Handle both seconds and milliseconds Unix timestamps.
    let seconds = if timestamp > 9_999_999_999 {
        timestamp / 1000
    } else {
        timestamp
    };

    match Local.timestamp_opt(seconds as i64, 0).single() {
        Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %z").to_string(),
        None => "Invalid date".to_string(),
    }
}