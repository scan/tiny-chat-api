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
    pub fn new(sender: &str, message: &str) -> Self {
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

    pub fn insert_message(&self, message: Message) -> anyhow::Result<()> {
        match self.0.write() {
            Ok(mut vec) => {
                vec.push(message);
                Ok(())
            }
            Err(error) => Err(anyhow::Error::msg(format!(
                "could not acquire write lock for message list: {:#?}",
                error
            ))),
        }
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
        let repository = Repository::new();
        let message = Message::new("sender", "message");
        repository.insert_message(message);
        assert_eq!(repository.get_messages(None).len(), 1);
    }
}
