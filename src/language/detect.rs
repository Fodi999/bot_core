use whatlang::{detect, Lang};

/// Определяет язык текста и возвращает его ISO-код (например, "EN", "RU", "JA")
pub fn detect_language(text: &str) -> Option<String> {
    detect(text).map(|info| info.lang().code().to_uppercase())
}

/// Проверка: является ли язык английским
pub fn is_english(text: &str) -> bool {
    detect(text).map_or(false, |info| info.lang() == Lang::Eng)
}

/// Проверка: является ли язык русским
pub fn is_russian(text: &str) -> bool {
    detect(text).map_or(false, |info| info.lang() == Lang::Rus)
}

/// Получает детальную информацию о языке (код + уверенность)
pub fn detect_language_with_confidence(text: &str) -> Option<(String, f64)> {
    detect(text).map(|info| {
        (info.lang().code().to_uppercase(), info.confidence())
    })
}

/// Определяет язык с fallback на английский, если определить не удалось
pub fn detect_language_or_default(text: &str) -> String {
    detect_language(text).unwrap_or_else(|| {
        // Простая эвристика: если есть кириллица - русский, иначе английский
        if text.chars().any(|c| ('\u{0400}'..='\u{04FF}').contains(&c)) {
            "RU".to_string()
        } else {
            "EN".to_string()
        }
    })
}

/// Проверяет, поддерживается ли язык ботом
pub fn is_supported_language(lang_code: &str) -> bool {
    matches!(lang_code, "EN" | "RU" | "DE" | "FR" | "ES" | "IT" | "JA" | "ZH")
}

/// Возвращает человекочитаемое название языка
pub fn get_language_name(lang_code: &str) -> &str {
    match lang_code {
        "EN" => "English",
        "RU" => "Русский",
        "DE" => "Deutsch", 
        "FR" => "Français",
        "ES" => "Español",
        "IT" => "Italiano",
        "JA" => "日本語",
        "ZH" => "中文",
        "PT" => "Português",
        "KO" => "한국어",
        "AR" => "العربية",
        "HI" => "हिन्दी",
        _ => "Unknown"
    }
}

/// Определяет основные языки в многоязычном тексте
pub fn detect_mixed_languages(text: &str) -> Vec<String> {
    let sentences: Vec<&str> = text.split(&['.', '!', '?'][..]).collect();
    let mut languages = Vec::new();
    
    for sentence in sentences {
        if sentence.trim().len() > 10 {
            if let Some(lang) = detect_language(sentence.trim()) {
                if !languages.contains(&lang) {
                    languages.push(lang);
                }
            }
        }
    }
    
    languages
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_language() {
        assert_eq!(detect_language("Hello world"), Some("EN".to_string()));
        assert_eq!(detect_language("Привет мир"), Some("RU".to_string()));
        assert_eq!(detect_language("Bonjour le monde"), Some("FR".to_string()));
    }

    #[test]
    fn test_language_checks() {
        assert!(is_english("Hello world"));
        assert!(is_russian("Привет мир"));
        assert!(!is_english("Привет мир"));
    }

    #[test]
    fn test_fallback() {
        let result = detect_language_or_default("abc");
        assert!(result == "EN" || result == "RU");
    }

    #[test]
    fn test_supported_languages() {
        assert!(is_supported_language("EN"));
        assert!(is_supported_language("RU"));
        assert!(!is_supported_language("XX"));
    }
}
