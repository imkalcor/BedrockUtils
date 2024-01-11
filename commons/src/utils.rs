use std::time::SystemTime;

/// Gets the current unix timestamp in milliseconds and returns it.
pub fn unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
