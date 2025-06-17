use reqwest;
use serde_json::Value;

pub async fn fetch_wikipedia_summary(query: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://en.wikipedia.org/api/rest_v1/page/summary/{}",
        urlencoding::encode(query)
    );
    
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    
    if response.status().is_success() {
        let json: Value = response.json().await?;
        if let Some(extract) = json["extract"].as_str() {
            Ok(extract.to_string())
        } else {
            Ok("No summary found.".to_string())
        }
    } else {
        Ok("Failed to fetch Wikipedia data.".to_string())
    }
}
