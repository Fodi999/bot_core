use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Message {
    pub user: String,
    pub text: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug)]
pub struct DialogContext {
    pub history: Vec<Message>,
}

impl DialogContext {
    pub fn new() -> Self {
        Self { history: Vec::new() }
    }

    pub fn add_message(&mut self, user: &str, text: &str) {
        self.history.push(Message {
            user: user.to_string(),
            text: text.to_string(),
            timestamp: Utc::now(),
        });
    }

    pub fn last_user_input(&self) -> Option<&Message> {
        self.history.iter().rev().find(|m| m.user == "user")
    }

    pub fn summary(&self) -> String {
        self.history
            .iter()
            .map(|m| format!("{}: {}", m.user, m.text))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
