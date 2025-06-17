use reqwest::blocking::get;
use quick_xml::de::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Feed {
    #[serde(rename = "entry", default)]
    entries: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
struct Entry {
    title: String,
    id: String,
    #[serde(rename = "summary")]
    abstract_text: Option<String>,
}

/// Ищет статьи на arXiv по запросу и возвращает заголовки с ссылками
pub fn search_arxiv(query: &str, max_results: usize) -> Result<Vec<(String, String)>, String> {
    let url = format!(
        "https://export.arxiv.org/api/query?search_query=all:{}&start=0&max_results={}",
        query.replace(" ", "+"),
        max_results
    );

    let body = get(&url)
        .map_err(|e| format!("Ошибка запроса arXiv: {}", e))?
        .text()
        .map_err(|e| format!("Ошибка чтения ответа arXiv: {}", e))?;

    let feed: Feed = from_str(&body).map_err(|e| format!("Ошибка парсинга XML: {}", e))?;

    Ok(feed
        .entries
        .into_iter()
        .map(|entry| (entry.title.trim().to_string(), entry.id))
        .collect())
}
