use reqwest::blocking::get;
use scraper::{Html, Selector};

/// Загружает HTML-страницу и возвращает её заголовок `<title>`
pub fn fetch_page_title(url: &str) -> Result<String, String> {
    let response = get(url)
        .map_err(|e| format!("Ошибка запроса: {}", e))?
        .text()
        .map_err(|e| format!("Ошибка чтения HTML: {}", e))?;

    let document = Html::parse_document(&response);
    let selector = Selector::parse("title").map_err(|e| e.to_string())?;

    if let Some(element) = document.select(&selector).next() {
        Ok(element.text().collect::<Vec<_>>().join(" ").trim().to_string())
    } else {
        Err("Заголовок не найден".into())
    }
}

/// Извлекает все абзацы (`<p>`) как список строк
pub fn extract_paragraphs(url: &str, max: usize) -> Result<Vec<String>, String> {
    let response = get(url)
        .map_err(|e| format!("Ошибка запроса: {}", e))?
        .text()
        .map_err(|e| format!("Ошибка чтения HTML: {}", e))?;

    let document = Html::parse_document(&response);
    let selector = Selector::parse("p").map_err(|e| e.to_string())?;

    let mut paragraphs = vec![];
    for el in document.select(&selector).take(max) {
        let text = el.text().collect::<Vec<_>>().join(" ").trim().to_string();
        if !text.is_empty() {
            paragraphs.push(text);
        }
    }

    Ok(paragraphs)
}
