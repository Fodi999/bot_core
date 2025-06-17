use whatlang::{detect, Lang};

/// Определяет язык текста и возвращает его ISO-код (например, "EN", "RU", "JA")
pub fn detect_language(text: &str) -> Option<String> {
    detect(text).map(|info| info.lang().code().to_uppercase())
}

/// Проверка: является ли язык английским
pub fn is_english(text: &str) -> bool {
    detect(text).map_or(false, |info| info.lang() == Lang::Eng)
}
