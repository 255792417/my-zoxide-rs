use crate::db::DirRecord;
use std::path;

pub fn calculate_score(path: &str, record: &DirRecord, keywords: &[String]) -> Option<f64> {
    let path_str = path.to_lowercase();
    let path_segments: Vec<&str> = path_str.split(path::MAIN_SEPARATOR).collect();

    let last_segment = path_segments.last().unwrap_or(&"");

    if !keywords
        .iter()
        .all(|kw| path_segments.contains(&kw.to_lowercase().as_str()))
    {
        return None;
    }

    let now = crate::get_now_time();
    let time_elapsed_secs = now.saturating_sub(record.last_accessed) as f64;
    let time_elapsed_hours = time_elapsed_secs / 3600.0;

    let rencencyy_factor = 1.0 / (1.0 + time_elapsed_hours).sqrt();
    let mut score = record.score * rencencyy_factor;

    if keywords.iter().any(|kw| kw.to_lowercase() == *last_segment) {
        score *= 2.0;
    }

    Some(score)
}
