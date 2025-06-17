use crate::knowledge::github::search_github_repos;

/// Определяет, относится ли вопрос к коду, и извлекает язык программирования
pub fn detect_code_query(input: &str) -> Option<String> {
    let keywords = ["example in", "пример на", "код на", "code in", "how to write"];

    for kw in keywords {
        if input.to_lowercase().contains(kw) {
            let parts: Vec<&str> = input.split_whitespace().collect();
            for (i, part) in parts.iter().enumerate() {
                if *part == "на" || *part == "in" {
                    if let Some(lang) = parts.get(i + 1) {
                        return Some(lang.to_string());
                    }
                }
            }
        }
    }

    None
}

/// Ищет примеры кода на GitHub по указанному языку
pub async fn fetch_code_examples(language: &str) -> Result<String, String> {
    let query = format!("{} example", language);
    let results = search_github_repos(&query, 3).await
        .map_err(|e| format!("Error searching GitHub: {}", e))?;
    
    if results.is_empty() {
        return Ok(format!("No code examples found for {}", language));
    }
    
    let formatted = results
        .into_iter()
        .map(|(name, desc, url)| format!("• {}\n  {}\n  {}", name, desc, url))
        .collect::<Vec<_>>()
        .join("\n\n");
    Ok(formatted)
}
