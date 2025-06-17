use reqwest::Client;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct DeepLTranslation {
    translations: Vec<Translation>,
}

#[derive(Debug, Deserialize)]
struct Translation {
    text: String,
}

/// Перевод текста на указанный язык (`EN`, `RU`, `FR`, и т.д.)
pub async fn translate_text(text: &str, target_lang: &str) -> Result<String, String> {
    let api_key = env::var("DEEPL_API_KEY").map_err(|e| format!("DEEPL_API_KEY: {}", e))?;
    let url = "https://api-free.deepl.com/v2/translate";

    let client = Client::new();
    let response = client
        .post(url)
        .form(&[
            ("auth_key", api_key.as_str()),
            ("text", text),
            ("target_lang", target_lang),
        ])
        .send()
        .await
        .map_err(|e| format!("Запрос DeepL не удался: {}", e))?;

    let data: DeepLTranslation = response
        .json()
        .await
        .map_err(|e| format!("Ошибка парсинга JSON DeepL: {}", e))?;

    Ok(data
        .translations
        .first()
        .map(|t| t.text.clone())
        .unwrap_or_else(|| "Ошибка: перевод отсутствует".into()))
}