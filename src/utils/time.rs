//! Time and timestamp utilities

use axum::http::StatusCode;
use chrono::{DateTime, Local, Utc};
use std::time::{SystemTime, UNIX_EPOCH};

/// Get current time in multiple formats
///
/// Returns a tuple of (unix_timestamp, utc_time, local_time)
pub fn get_timestamps() -> Result<(u64, String, String), StatusCode> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let unix_timestamp = now.as_secs();

    let datetime: DateTime<Utc> = DateTime::from_timestamp(unix_timestamp as i64, 0)
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let utc_time = datetime.format("%Y-%m-%d %H:%M:%S UTC").to_string();
    let local_time = datetime
        .with_timezone(&Local)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();

    Ok((unix_timestamp, utc_time, local_time))
}
