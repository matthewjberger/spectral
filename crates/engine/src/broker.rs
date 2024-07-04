#![allow(dead_code)]

use async_std::future::timeout;
use async_std::sync::{Arc, Mutex};
use futures::channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
use futures::stream::StreamExt;
use std::{collections::HashMap, time::Duration};

pub type Subscriber<T> = UnboundedSender<T>;

pub struct Broker<T> {
    subscribers: Arc<Mutex<HashMap<String, Vec<Subscriber<T>>>>>,
}
impl<T: Clone + Send + 'static> Default for Broker<T> {
    fn default() -> Self {
        Self {
            subscribers: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl<T: Clone + Send + 'static> Broker<T> {
    pub async fn subscribe(&self, topic: &str, tx: Subscriber<T>) {
        let mut subscribers = self.subscribers.lock().await;
        subscribers
            .entry(topic.to_string())
            .or_insert_with(Vec::new)
            .push(tx);
    }

    pub async fn publish(&self, topic: &str, msg: T) {
        let subscribers = self.subscribers.lock().await;
        if let Some(subs) = subscribers.get(topic) {
            for sub in subs {
                let _ = sub.unbounded_send(msg.clone());
            }
        }
    }
}

pub struct Client<T> {
    broker: Arc<Broker<T>>,
    receiver: Arc<Mutex<UnboundedReceiver<T>>>,
    sender: Subscriber<T>,
}

impl<T: Clone + Send + 'static> Client<T> {
    pub fn new(broker: Arc<Broker<T>>) -> Self {
        let (tx, rx) = unbounded();
        Self {
            broker,
            receiver: Arc::new(Mutex::new(rx)),
            sender: tx,
        }
    }

    pub async fn subscribe(&self, topic: &str) {
        self.broker.subscribe(topic, self.sender.clone()).await;
    }

    pub async fn next(&self) -> Option<T> {
        let mut receiver = self.receiver.lock().await;
        receiver.next().await
    }

    pub async fn try_next_message(&self, duration: Duration) -> Option<T> {
        let mut receiver = self.receiver.lock().await;
        match timeout(duration, receiver.next()).await {
            Ok(Some(msg)) => Some(msg),
            Ok(None) | Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[async_std::test]
    async fn test_pubsub() {
        let broker = Arc::new(Broker::default());

        let client1 = Client::new(broker.clone());
        let client2 = Client::new(broker.clone());

        client1.subscribe("topic1").await;
        client2.subscribe("topic1").await;

        broker.publish("topic1", "message1").await;

        assert_eq!(client1.next().await.unwrap(), "message1");
        assert_eq!(client2.next().await.unwrap(), "message1");
    }

    #[async_std::test]
    async fn test_try_next_message() {
        let broker = Arc::new(Broker::default());
        let client = Client::new(broker.clone());

        client.subscribe("topic1").await;

        // Test timeout with no message published
        let msg = client.try_next_message(Duration::from_secs(1)).await;
        assert!(msg.is_none());

        // Test with a message published
        broker.publish("topic1", "message1").await;
        let msg = client.try_next_message(Duration::from_secs(1)).await;
        assert_eq!(msg.unwrap(), "message1");
    }
}
