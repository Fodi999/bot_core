use chrono::prelude::*;

/// Текущая временная метка в читаемом формате
pub fn current_timestamp() -> String {
    Utc::now().to_rfc3339()
}

/// Обрезает строку до max символов с «...», если превышает
pub fn truncate(text: &str, max_len: usize) -> String {
    if text.len() > max_len {
        format!("{}...", &text[..max_len])
    } else {
        text.to_string()
    }
}

/// Делит строку на предложения
pub fn split_into_sentences(text: &str) -> Vec<String> {
    text.split_terminator(|c| c == '.' || c == '?' || c == '!')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}
