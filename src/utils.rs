use similar::{ChangeTag, TextDiff};

pub fn calculate_path_similarity(path1: &str, path2: &str) -> f64 {
    let diff = TextDiff::from_chars(path1, path2);
    let matching_chars: usize = diff
        .iter_all_changes()
        .filter(|change| change.tag() == ChangeTag::Equal)
        .map(|change| change.to_string().len())
        .sum();
    
    let max_len = path1.len().max(path2.len());
    if max_len == 0 {
        return 0.0;
    }
    
    (matching_chars as f64 / max_len as f64) * 100.0
}