use crate::db::DirRecord;
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use std::path;

fn frecency_score(record: &DirRecord) -> f64 {
    let now = crate::get_now_time();
    let time_elapsed_secs = now.saturating_sub(record.last_accessed) as f64;
    let time_elapsed_hours = time_elapsed_secs / 3600.0;

    record.score / (1.0 + time_elapsed_hours).sqrt()
}

pub fn calculate_score(path: &str, record: &DirRecord, keyword: &str) -> Option<f64> {
    let path_str = path.to_lowercase();
    let last_segment = path_str.split(path::MAIN_SEPARATOR).last().unwrap_or(&"");

    let matcher = SkimMatcherV2::default();

    let mut score: f64 = matcher
        .fuzzy_match(last_segment, keyword)?
        .max(matcher.fuzzy_match(&path_str, keyword)?) as f64;

    let frecency = frecency_score(record);
    score *= frecency;

    Some(score)
}
