use crate::knowledge::github::search_github_repos;

/// Определяет, относится ли вопрос к коду, и извлекает язык программирования
pub fn detect_code_query(input: &str) -> Option<String> {
    let input_lower = input.to_lowercase();
    
    // Расширенные ключевые слова для поиска
    let keywords = [
        "example in", "пример на", "код на", "code in", "how to write",
        "show me", "покажи", "найди код", "examples for", "tutorial",
        "learn", "изучить", "syntax", "синтаксис"
    ];

    // Популярные языки программирования
    let languages = [
        "rust", "python", "javascript", "java", "c++", "cpp", "c#", "csharp",
        "go", "kotlin", "swift", "php", "ruby", "typescript", "scala",
        "haskell", "clojure", "dart", "r", "matlab", "perl", "lua",
        "assembly", "bash", "powershell", "sql", "html", "css"
    ];

    // Проверяем наличие ключевых слов
    let has_keywords = keywords.iter().any(|kw| input_lower.contains(kw));
    
    if has_keywords {
        // Ищем язык программирования после ключевых слов
        let parts: Vec<&str> = input_lower.split_whitespace().collect();
        for (i, part) in parts.iter().enumerate() {
            if part == &"на" || part == &"in" || part == &"for" {
                if let Some(lang) = parts.get(i + 1) {
                    let normalized_lang = normalize_language_name(lang);
                    if languages.contains(&normalized_lang.as_str()) {
                        return Some(normalized_lang);
                    }
                }
            }
        }
        
        // Если не нашли язык после предлогов, ищем в любом месте
        for word in parts {
            let normalized = normalize_language_name(word);
            if languages.contains(&normalized.as_str()) {
                return Some(normalized);
            }
        }
    }
    
    // Прямое упоминание языка без ключевых слов
    for lang in &languages {
        if input_lower.contains(lang) {
            return Some(normalize_language_name(lang));
        }
    }

    None
}

/// Нормализует название языка программирования
fn normalize_language_name(lang: &str) -> String {
    match lang.to_lowercase().as_str() {
        "js" | "javascript" => "javascript".to_string(),
        "ts" | "typescript" => "typescript".to_string(),
        "py" | "python" => "python".to_string(),
        "cpp" | "c++" => "cpp".to_string(),
        "cs" | "c#" | "csharp" => "csharp".to_string(),
        "go" | "golang" => "go".to_string(),
        "rs" | "rust" => "rust".to_string(),
        _ => lang.to_lowercase()
    }
}

/// Ищет примеры кода на GitHub по указанному языку
pub async fn fetch_code_examples(language: &str) -> Result<String, String> {
    let normalized_lang = normalize_language_name(language);
    let query = format!("{} example tutorial", normalized_lang);
    
    println!("🔍 Поиск примеров кода: {}", query);
    
    let results = search_github_repos(&query, 5).await
        .map_err(|e| format!("Ошибка поиска в GitHub: {}", e))?;
    
    if results.is_empty() {
        return Ok(format!("❌ Примеры кода для **{}** не найдены.\n\nПопробуйте поискать:\n• Tutorials\n• Documentation\n• Stack Overflow", 
                         normalized_lang.to_uppercase()));
    }
    
    let mut formatted = format!("🚀 **Примеры кода на {}:**\n\n", normalized_lang.to_uppercase());
    
    for (i, (name, desc, url)) in results.iter().enumerate() {
        formatted.push_str(&format!(
            "{}. 📂 **{}**\n   💡 {}\n   🔗 {}\n\n",
            i + 1,
            name,
            if desc.is_empty() { "Нет описания" } else { desc },
            url
        ));
    }
    
    formatted.push_str("💡 **Совет:** Изучите README файлы в этих репозиториях для лучшего понимания!");
    
    Ok(formatted)
}

/// Генерирует простой пример кода для популярных языков
pub fn generate_basic_example(language: &str) -> Option<String> {
    let normalized = normalize_language_name(language);
    
    match normalized.as_str() {
        "rust" => Some(format!(
            "🦀 **Базовый пример Rust:**\n\n```rust\nfn main() {{\n    println!(\"Hello, World!\");\n    \n    let name = \"Rust\";\n    println!(\"Привет от {{}}!\", name);\n}}\n```"
        )),
        "python" => Some(format!(
            "🐍 **Базовый пример Python:**\n\n```python\n# Hello World\nprint(\"Hello, World!\")\n\n# Переменные\nname = \"Python\"\nprint(f\"Привет от {{name}}!\")\n```"
        )),
        "javascript" => Some(format!(
            "⚡ **Базовый пример JavaScript:**\n\n```javascript\n// Hello World\nconsole.log(\"Hello, World!\");\n\n// Переменные\nconst name = \"JavaScript\";\nconsole.log(`Привет от ${{name}}!`);\n```"
        )),
        "go" => Some(format!(
            "🔵 **Базовый пример Go:**\n\n```go\npackage main\n\nimport \"fmt\"\n\nfunc main() {{\n    fmt.Println(\"Hello, World!\")\n    \n    name := \"Go\"\n    fmt.Printf(\"Привет от %s!\\n\", name)\n}}\n```"
        )),
        _ => None
    }
}