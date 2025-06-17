use reqwest;
use serde_json::Value;

pub async fn search_github_repos(query: &str, max_results: usize) -> Result<Vec<(String, String, String)>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.github.com/search/repositories?q={}&sort=stars&order=desc&per_page={}",
        urlencoding::encode(query),
        max_results
    );
    
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Bot-Auraya/1.0")
        .send()
        .await?;
    
    if response.status().is_success() {
        let json: Value = response.json().await?;
        let mut repos = Vec::new();
        
        if let Some(items) = json["items"].as_array() {
            for item in items.iter().take(max_results) {
                let name = item["name"].as_str().unwrap_or("Unknown").to_string();
                let description = item["description"].as_str().unwrap_or("No description").to_string();
                let url = item["html_url"].as_str().unwrap_or("").to_string();
                repos.push((name, description, url));
            }
        }
        
        if repos.is_empty() {
            return Err("No repositories found".into());
        }
        
        Ok(repos)
    } else {
        Err(format!("GitHub API returned status: {}", response.status()).into())
    }
}