use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct WikipediaSummary {
    title: String,
    extract: String,
    #[serde(default)]
    pageid: Option<u64>,
    #[serde(default)] 
    type_field: Option<String>,
}

/// Получает краткое описание из Wikipedia
pub async fn fetch_wikipedia_summary(query: &str) -> Result<String, String> {
    // Очищаем запрос от лишних слов
    let clean_query = clean_wikipedia_query(query);
    
    // Пробуем сначала REST API
    match fetch_from_rest_api(&clean_query).await {
        Ok(result) => Ok(result),
        Err(e) => {
            println!("⚠️ REST API не сработал: {}, пробуем Action API", e);
            fetch_from_action_api(&clean_query).await
        }
    }
}

/// Получает данные через REST API Wikipedia
async fn fetch_from_rest_api(query: &str) -> Result<String, String> {
    let url = format!(
        "https://en.wikipedia.org/api/rest_v1/page/summary/{}",
        urlencoding::encode(query)
    );
    
    println!("🔍 Wikipedia REST URL: {}", url);
    
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("Auraya-Bot/1.0 (https://github.com/auraya-bot)")
        .build()
        .map_err(|e| format!("HTTP client error: {}", e))?;

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Wikipedia request error: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Wikipedia REST API status {}: {}", status, error_text));
    }

    let summary: WikipediaSummary = response
        .json()
        .await
        .map_err(|e| format!("JSON parsing error: {}", e))?;

    if summary.extract.is_empty() || summary.extract.contains("may refer to:") {
        return Err("Disambiguation page or empty summary".to_string());
    }

    Ok(format!("📖 **{}**\n\n{}", summary.title, summary.extract))
}

/// Получает данные через Action API Wikipedia (запасной вариант)
async fn fetch_from_action_api(query: &str) -> Result<String, String> {
    let url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&format=json&titles={}&prop=extracts&exintro&explaintext&exsectionformat=plain",
        urlencoding::encode(query)
    );
    
    println!("🔍 Wikipedia Action URL: {}", url);
    
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("Auraya-Bot/1.0 (https://github.com/auraya-bot)")
        .build()
        .map_err(|e| format!("HTTP client error: {}", e))?;

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Wikipedia request error: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Wikipedia Action API returned status: {}", response.status()));
    }

    let wiki_response: WikipediaResponse = response
        .json()
        .await
        .map_err(|e| format!("JSON parsing error: {}", e))?;

    if let Some(query) = wiki_response.query {
        for (_, page) in query.pages {
            if let Some(extract) = page.extract {
                if !extract.is_empty() && !extract.contains("may refer to:") {
                    let title = page.title.unwrap_or_else(|| "Wikipedia".to_string());
                    return Ok(format!("📖 **{}**\n\n{}", title, extract));
                }
            }
        }
    }

    Err("No summary available".to_string())
}

#[derive(Debug, Deserialize)]
struct WikipediaResponse {
    query: Option<WikipediaQuery>,
}

#[derive(Debug, Deserialize)]
struct WikipediaQuery {
    pages: std::collections::HashMap<String, WikipediaPage>,
}

#[derive(Debug, Deserialize)]
struct WikipediaPage {
    extract: Option<String>,
    title: Option<String>,
}

/// Очищает запрос для Wikipedia
fn clean_wikipedia_query(query: &str) -> String {
    let query_lower = query.to_lowercase();
    
    // Убираем лишние слова
    let cleaned = query_lower
        .replace("what is ", "")
        .replace("что такое ", "") 
        .replace("tell me about ", "")
        .replace("расскажи о ", "")
        .replace("explain ", "")
        .replace("объясни ", "")
        .replace("?", "")
        .replace("!", "")
        .trim()
        .to_string();
    
    // Специальные случаи для улучшения поиска
    let result = match cleaned.as_str() {
        "ai" | "ии" => "artificial_intelligence".to_string(),
        "ml" => "machine_learning".to_string(),
        "js" => "javascript".to_string(),
        "css" => "cascading_style_sheets".to_string(),
        "html" => "html".to_string(),
        "cpp" | "c++" => "c++".to_string(),
        _ => cleaned.replace(" ", "_")
    };
    
    println!("🔧 Очищенный запрос: '{}' -> '{}'", query, result);
    result
}

/// Пытается найти похожие статьи, если точного совпадения нет
pub async fn search_wikipedia_articles(query: &str, limit: usize) -> Result<Vec<String>, String> {
    let url = format!(
        "https://en.wikipedia.org/w/api.php?action=opensearch&search={}&limit={}&format=json",
        urlencoding::encode(query),
        limit
    );
    
    println!("🔍 Wikipedia Search URL: {}", url);
    
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("Auraya-Bot/1.0")
        .build()
        .map_err(|e| format!("HTTP client error: {}", e))?;

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Wikipedia search error: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Wikipedia search API status: {}", response.status()));
    }

    let search_results: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("JSON parsing error: {}", e))?;

    if let Some(titles) = search_results.get(1).and_then(|v| v.as_array()) {
        let results: Vec<String> = titles
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .take(limit)
            .collect();
        
        Ok(results)
    } else {
        Err("No search results found".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_query() {
        assert_eq!(clean_wikipedia_query("What is Rust?"), "rust");
        assert_eq!(clean_wikipedia_query("что такое Python"), "python");
        assert_eq!(clean_wikipedia_query("artificial intelligence"), "artificial_intelligence");
        assert_eq!(clean_wikipedia_query("What is AI?"), "artificial_intelligence");
        assert_eq!(clean_wikipedia_query("C++"), "c++");
    }

    #[tokio::test]
    async fn test_wikipedia_search() {
        let results = search_wikipedia_articles("rust programming", 3).await;
        assert!(results.is_ok());
    }
}
