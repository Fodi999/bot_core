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
    // Проверяем наличие API ключа
    let api_key = match env::var("DEEPL_API_KEY") {
        Ok(key) if !key.is_empty() => key,
        _ => {
            println!("⚠️ DEEPL_API_KEY не найден или пуст - переводчик отключен");
            return Err("DeepL API ключ недоступен".to_string());
        }
    };

    let url = "https://api-free.deepl.com/v2/translate";

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10)) // Тайм-аут 10 секунд
        .build()
        .map_err(|e| format!("Ошибка создания HTTP клиента: {}", e))?;

    let response = client
        .post(url)
        .header("User-Agent", "Bot-Auraya/1.0")
        .form(&[
            ("auth_key", api_key.as_str()),
            ("text", text),
            ("target_lang", target_lang),
        ])
        .send()
        .await
        .map_err(|e| format!("Запрос DeepL не удался: {}", e))?;

    // Проверяем статус ответа
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("DeepL API ошибка {}: {}", status, error_text));
    }

    let response_text = response.text().await
        .map_err(|e| format!("Ошибка чтения ответа DeepL: {}", e))?;

    // Логируем ответ для отладки (только первые 200 символов)
    println!("🔍 DeepL ответ: {}", &response_text[..response_text.len().min(200)]);

    let data: DeepLTranslation = serde_json::from_str(&response_text)
        .map_err(|e| format!("Ошибка парсинга JSON DeepL: {} | Ответ: {}", e, response_text))?;

    if data.translations.is_empty() {
        return Err("DeepL вернул пустой список переводов".to_string());
    }

    Ok(data.translations[0].text.clone())
}

/// Упрощенный переводчик-заглушка для случаев, когда DeepL недоступен
pub fn simple_translate_fallback(text: &str, target_lang: &str) -> String {
    match target_lang {
        "RU" => {
            // Простейшие замены для демонстрации
            text.replace("Hello", "Привет")
                .replace("Thank you", "Спасибо")
                .replace("Goodbye", "До свидания")
                .replace("Error", "Ошибка")
        }
        "EN" => {
            text.replace("Привет", "Hello")
                .replace("Спасибо", "Thank you")
                .replace("До свидания", "Goodbye")
                .replace("Ошибка", "Error")
        }
        _ => text.to_string() // Возвращаем оригинал для неподдерживаемых языков
    }
}