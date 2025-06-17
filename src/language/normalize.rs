use regex::Regex;

/// Приводит строку к нормализованному виду:
/// - убирает лишние пробелы,
/// - переводит в нижний регистр,
/// - удаляет управляющие символы
pub fn normalize(text: &str) -> String {
    let text = text.trim().to_lowercase();
    let re_space = Regex::new(r"\s+").unwrap();
    let cleaned = re_space.replace_all(&text, " ");
    cleaned.to_string()
}

/// Удаляет пунктуацию (если нужно)
pub fn remove_punctuation(text: &str) -> String {
    let re_punct = Regex::new(r"[[:punct:]]").unwrap();
    re_punct.replace_all(text, "").to_string()
}
