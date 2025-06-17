use regex::Regex;
use std::sync::OnceLock;

// Ленивая инициализация регулярных выражений для лучшей производительности
static SPACE_REGEX: OnceLock<Regex> = OnceLock::new();
static PUNCT_REGEX: OnceLock<Regex> = OnceLock::new();
static EMOJI_REGEX: OnceLock<Regex> = OnceLock::new();

/// Приводит строку к нормализованному виду:
/// - убирает лишние пробелы,
/// - переводит в нижний регистр,
/// - удаляет управляющие символы
pub fn normalize(text: &str) -> String {
    let text = text.trim().to_lowercase();
    let re_space = SPACE_REGEX.get_or_init(|| Regex::new(r"\s+").unwrap());
    let cleaned = re_space.replace_all(&text, " ");
    cleaned.to_string()
}

/// Удаляет пунктуацию (если нужно)
pub fn remove_punctuation(text: &str) -> String {
    let re_punct = PUNCT_REGEX.get_or_init(|| Regex::new(r"[[:punct:]]").unwrap());
    re_punct.replace_all(text, "").to_string()
}

/// Удаляет эмодзи и специальные символы
pub fn remove_emoji(text: &str) -> String {
    let re_emoji = EMOJI_REGEX.get_or_init(|| {
        Regex::new(r"[\u{1F600}-\u{1F64F}]|[\u{1F300}-\u{1F5FF}]|[\u{1F680}-\u{1F6FF}]|[\u{1F1E0}-\u{1F1FF}]|[\u{2600}-\u{26FF}]|[\u{2700}-\u{27BF}]").unwrap()
    });
    re_emoji.replace_all(text, "").to_string()
}

/// Очищает текст для анализа (убирает все лишнее)
pub fn clean_for_analysis(text: &str) -> String {
    let text = remove_emoji(text);
    let text = remove_punctuation(&text);
    normalize(&text)
}

/// Извлекает только слова (буквы и цифры)
pub fn extract_words(text: &str) -> Vec<String> {
    let normalized = normalize(text);
    normalized
        .split_whitespace()
        .filter(|word| !word.is_empty() && word.len() > 1)
        .map(|word| word.to_string())
        .collect()
}

/// Подсчитывает количество слов
pub fn word_count(text: &str) -> usize {
    extract_words(text).len()
}

/// Обрезает текст до определенного количества слов
pub fn truncate_words(text: &str, max_words: usize) -> String {
    let words = extract_words(text);
    if words.len() <= max_words {
        text.to_string()
    } else {
        words[..max_words].join(" ") + "..."
    }
}

/// Проверяет, содержит ли текст только латиницу
pub fn is_latin_only(text: &str) -> bool {
    text.chars().all(|c| c.is_ascii() || c.is_whitespace())
}

/// Проверяет, содержит ли текст кириллицу
pub fn contains_cyrillic(text: &str) -> bool {
    text.chars().any(|c| ('\u{0400}'..='\u{04FF}').contains(&c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() {
        assert_eq!(normalize("  Привет   Мир!  "), "привет мир!");
        assert_eq!(normalize("Hello\t\nWorld"), "hello world");
    }

    #[test]
    fn test_word_count() {
        assert_eq!(word_count("Hello world test"), 3);
        assert_eq!(word_count("Привет мир"), 2);
    }

    #[test]
    fn test_language_detection() {
        assert!(is_latin_only("Hello world"));
        assert!(!is_latin_only("Привет мир"));
        assert!(contains_cyrillic("Привет world"));
    }
}
