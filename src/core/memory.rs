use chrono::{DateTime, Utc};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct MemoryItem {
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub source: String, // например: "user", "wiki", "github", "inferred"
}

#[derive(Debug)]
pub struct Memory {
    pub short_term: VecDeque<MemoryItem>, // последние 10 фраз
    pub long_term: Vec<MemoryItem>,       // знания и факты
    pub max_short_term: usize,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            short_term: VecDeque::new(),
            long_term: Vec::new(),
            max_short_term: 10,
        }
    }

    pub fn remember(&mut self, content: &str, source: &str) {
        let item = MemoryItem {
            content: content.to_string(),
            created_at: Utc::now(),
            source: source.to_string(),
        };
        self.short_term.push_back(item.clone());
        if self.short_term.len() > self.max_short_term {
            self.short_term.pop_front();
        }
        self.long_term.push(item);
    }

    pub fn last_facts(&self, count: usize) -> Vec<String> {
        self.long_term
            .iter()
            .rev()
            .take(count)
            .map(|i| i.content.clone())
            .collect()
    }

    pub fn find(&self, keyword: &str) -> Vec<&MemoryItem> {
        self.long_term
            .iter()
            .filter(|item| item.content.contains(keyword))
            .collect()
    }

    pub fn context_summary(&self) -> String {
        self.short_term
            .iter()
            .map(|item| format!("- {} [{}]", item.content, item.source))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
