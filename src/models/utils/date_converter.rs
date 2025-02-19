use chrono::{DateTime, Local};

pub fn format_timestamp(ts: i64) -> String {
    if ts == 0 {
        "N/A".to_string()
    } else {
        if let Some(dt) = DateTime::from_timestamp(ts, 0) {
            dt.with_timezone(&Local)
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string()
        } else {
            "Invalid timestamp".to_string()
        }
    }
}