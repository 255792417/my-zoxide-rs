pub mod db;
pub mod engine;

use anyhow::Result;
use std::fs;
pub fn get_abs_path(path: &str) -> Result<String> {
    fs::canonicalize(path)
        .map_err(|e| anyhow::anyhow!("Failed to get absolute path: {}", e))
        .and_then(|p| {
            p.into_os_string()
                .into_string()
                .map_err(|_| anyhow::anyhow!("Failed to convert path to string"))
        })
}

use std::time::{SystemTime, UNIX_EPOCH};
fn get_now_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
