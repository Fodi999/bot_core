use reqwest;
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
pub async fn search_arxiv(query: &str, max_results: usize) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://export.arxiv.org/api/query?search_query=all:{}&start=0&max_results={}",
        query.replace(" ", "+"),
        max_results
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Bot-Auraya/1.0")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("arXiv API returned status: {}", response.status()).into());
    }

    let body = response.text().await?;
    let feed: Feed = from_str(&body)?;

    if feed.entries.is_empty() {
        return Err("No arXiv articles found".into());
    }

    Ok(feed
        .entries
        .into_iter()
        .map(|entry| (entry.title.trim().to_string(), entry.id))
        .collect())
}
