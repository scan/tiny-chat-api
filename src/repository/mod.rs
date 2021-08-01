use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    sync::{Arc, RwLock},
};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct Message {
    id: uuid::Uuid,
    pub sender: String,
    pub message: String,
    pub sent_at: DateTime<Utc>,
}

impl Message {
    fn new(sender: &str, message: &str) -> Self {
        Message {
            id: uuid::Uuid::new_v4(),
            sender: sender.to_owned(),
            message: message.to_owned(),
            sent_at: Utc::now(),
        }
    }
}

impl PartialOrd for Message {
    fn partial_cmp(&self, other: &Message) -> Option<Ordering> {
        self.sent_at.partial_cmp(&other.sent_at)
    }
}

impl Ord for Message {
    fn cmp(&self, other: &Message) -> Ordering {
        self.sent_at.cmp(&other.sent_at)
    }
}

#[derive(Debug, Clone)]
pub struct Repository(Arc<RwLock<Vec<Message>>>);

impl Repository {
    pub fn new() -> Self {
        Repository(Arc::new(RwLock::new(vec![])))
    }

    pub fn insert_message(&mut self, message: Message) {
        self.0.write().unwrap().push(message);
    }

    pub fn get_messages(&self, after: Option<chrono::DateTime<chrono::Utc>>) -> Vec<Message> {
        let messages = self.0.read().unwrap().clone();

        if let Some(timestamp) = after {
            messages
                .into_iter()
                .filter(|m| m.sent_at > timestamp)
                .collect()
        } else {
            messages
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_message() {
        let mut repository = Repository::new();
        let message = Message::new("sender", "message");
        repository.insert_message(message);
        assert_eq!(repository.get_messages(None).len(), 1);
    }
}
