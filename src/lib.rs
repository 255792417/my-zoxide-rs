pub mod db;
pub mod engine;

use std::fs;
pub fn get_abs_path(path: &str) -> Option<String> {
    fs::canonicalize(path)
        .ok()
        .and_then(|p| p.to_str().map(|s| s.to_string()))
}

use std::time::{SystemTime, UNIX_EPOCH};
fn get_now_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
