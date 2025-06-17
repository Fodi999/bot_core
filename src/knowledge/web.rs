use reqwest;
use scraper::{Html, Selector};

/// Загружает HTML-страницу и возвращает её заголовок `<title>`
pub async fn fetch_page_title(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "Bot-Auraya/1.0")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("HTTP status: {}", response.status()).into());
    }

    let html = response.text().await?;
    let document = Html::parse_document(&html);
    let selector = Selector::parse("title").map_err(|e| e.to_string())?;

    if let Some(element) = document.select(&selector).next() {
        let title = element.text().collect::<Vec<_>>().join(" ").trim().to_string();
        if title.is_empty() {
            return Err("Заголовок пустой".into());
        }
        Ok(title)
    } else {
        Err("Заголовок не найден".into())
    }
}

/// Извлекает все абзацы (`<p>`) как список строк
pub async fn extract_paragraphs(url: &str, max: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "Bot-Auraya/1.0")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("HTTP status: {}", response.status()).into());
    }

    let html = response.text().await?;
    let document = Html::parse_document(&html);
    let selector = Selector::parse("p").map_err(|e| e.to_string())?;

    let mut paragraphs = vec![];
    for el in document.select(&selector).take(max) {
        let text = el.text().collect::<Vec<_>>().join(" ").trim().to_string();
        if !text.is_empty() && text.len() > 10 {  // Фильтруем слишком короткие абзацы
            paragraphs.push(text);
        }
    }

    if paragraphs.is_empty() {
        return Err("Абзацы не найдены".into());
    }

    Ok(paragraphs)
}

/// Безопасно извлекает краткое содержимое страницы (заголовок + первые абзацы)
pub async fn extract_page_summary(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let title = fetch_page_title(url).await?;
    
    match extract_paragraphs(url, 3).await {
        Ok(paragraphs) => {
            let summary = paragraphs.join("\n\n");
            Ok(format!("**{}**\n\n{}", title, summary))
        }
        Err(_) => {
            // Если абзацы не найдены, возвращаем хотя бы заголовок
            Ok(format!("**{}**\n\nИнформация о содержимом недоступна.", title))
        }
    }
}
