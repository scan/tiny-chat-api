use chrono::prelude::*;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    sync::{Arc, PoisonError, RwLock},
};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, GraphQLObject)]
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
pub struct Repository {
    messages: Arc<RwLock<Vec<Message>>>,
    transmitters: Arc<RwLock<Vec<crossbeam_channel::Sender<Message>>>>,
}

fn unpoison_error<T>(e: PoisonError<T>) -> anyhow::Error {
    anyhow::Error::msg(format!("could not acquire write lock: {:#?}", e))
}

impl Repository {
    pub fn new() -> Self {
        Repository {
            messages: Arc::new(RwLock::new(vec![])),
            transmitters: Arc::new(RwLock::new(vec![])),
        }
    }

    pub fn insert_message(&self, message: Message) -> anyhow::Result<()> {
        self.messages
            .write()
            .map_err(unpoison_error)?
            .push(message.clone());

        self.transmitters
            .read()
            .map_err(unpoison_error)?
            .iter()
            .try_for_each(|t| t.send(message.clone()))?;

        Ok(())
    }

    pub fn get_messages(
        &self,
        after: Option<chrono::DateTime<chrono::Utc>>,
    ) -> anyhow::Result<Vec<Message>> {
        let messages = self.messages.read().map_err(unpoison_error)?;

        let filtered_messages: Vec<Message> = if let Some(timestamp) = after {
            messages
                .clone()
                .into_iter()
                .filter(|m| m.sent_at >= timestamp)
                .collect()
        } else {
            messages.clone()
        };

        Ok(filtered_messages)
    }

    pub fn register_listener(&self) -> anyhow::Result<crossbeam_channel::Receiver<Message>> {
        let (tx, rx) = crossbeam_channel::unbounded();
        self.transmitters.write().map_err(unpoison_error)?.push(tx);

        Ok(rx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use enclose::enclose;
    use std::thread;

    #[test]
    fn test_insert_message() {
        let repository = Repository::new();
        let message = Message::new("sender", "message");
        assert!(repository.insert_message(message).is_ok());
        assert!(repository.get_messages(None).is_ok());
        assert_eq!(repository.get_messages(None).unwrap().len(), 1);
    }

    #[test]
    fn test_parallel_insert_message() {
        let repository = Repository::new();

        let handles: Vec<thread::JoinHandle<_>> = (0..10)
            .map(|i| {
                let repository = repository.clone();
                thread::spawn(enclose!((repository) move || {
                    let message = Message::new("sender", &format!("message {}", i));

                    assert!(repository.insert_message(message).is_ok());
                }))
            })
            .collect();

        for handle in handles {
            assert!(handle.join().is_ok());
        }

        assert!(repository.get_messages(None).is_ok());
        assert_eq!(repository.get_messages(None).unwrap().len(), 10);
    }
}
