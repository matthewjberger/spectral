use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, RwLock, Weak},
};
use uuid::Uuid;

#[derive(Default)]
pub struct Broker<T: Clone> {
    subscribers: Arc<RwLock<HashMap<String, Vec<Weak<RwLock<Client<T>>>>>>>,
}

impl<T: Clone> Broker<T> {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn subscribe(&self, topic: &str, client: &Arc<RwLock<Client<T>>>) {
        let client_weak = Arc::downgrade(client);
        self.subscribers
            .write()
            .unwrap()
            .entry(topic.to_string())
            .or_default()
            .push(client_weak);
    }

    pub fn unsubscribe(&self, topic: &str, client_id: Uuid) -> Result<(), &'static str> {
        let mut subscribers = self.subscribers.write().unwrap();
        if let Some(subscribers) = subscribers.get_mut(topic) {
            subscribers.retain(|subscriber| {
                if let Some(subscriber) = subscriber.upgrade() {
                    subscriber.read().unwrap().id() != client_id
                } else {
                    false
                }
            });
            Ok(())
        } else {
            Err("TopicNotFound")
        }
    }

    pub fn publish(&self, topic: &str, message: T) {
        let mut subscribers = self.subscribers.write().unwrap();
        if let Some(subscribers) = subscribers.get_mut(topic) {
            subscribers.retain(|subscriber_weak| {
                if let Some(subscriber_strong) = subscriber_weak.upgrade() {
                    let mut subscriber = subscriber_strong.write().unwrap();
                    let ring_buffer_size = subscriber.ring_buffer_size();
                    if subscriber.event_queue.len() == ring_buffer_size {
                        subscriber.event_queue.pop_front();
                    }
                    subscriber.event_queue.push_back(message.clone());
                    true
                } else {
                    false
                }
            });
            if subscribers.is_empty() {
                self.subscribers.write().unwrap().remove(topic);
            }
        }
    }
}

pub type ClientHandle<T> = Arc<RwLock<Client<T>>>;

pub struct Client<T: Clone> {
    id: Uuid,
    event_queue: VecDeque<T>,
    ring_buffer_size: usize,
}

impl<T: Clone> Default for Client<T> {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            event_queue: VecDeque::new(),
            ring_buffer_size: 100,
        }
    }
}

impl<T: Clone> Client<T> {
    pub fn new() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self::default()))
    }

    pub fn with_ring_buffer_size(size: usize) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            ring_buffer_size: size,
            ..Default::default()
        }))
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn ring_buffer_size(&self) -> usize {
        self.ring_buffer_size
    }

    pub fn next_message(&mut self) -> Option<T> {
        self.event_queue.pop_front()
    }

    pub fn peek_message(&self) -> Option<T> {
        self.event_queue.front().cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::{Broker, Client};

    #[derive(Debug, Clone, PartialEq)]
    pub struct Message {
        content: String,
    }

    impl Message {
        pub fn new(content: &str) -> Self {
            Self {
                content: content.to_string(),
            }
        }
    }

    #[test]
    fn test_single_client_receive_message() {
        let broker = Broker::new();
        let client = Client::new();
        broker.subscribe("topic1", &client);
        broker.publish("topic1", Message::new("hello world"));
        assert_eq!(
            client.write().unwrap().next_message().unwrap().content,
            "hello world"
        );
    }

    #[test]
    fn test_multiple_subscribers_receive_message() {
        let broker = Broker::new();
        let client1 = Client::new();
        let client2 = Client::new();
        broker.subscribe("topic1", &client1);
        broker.subscribe("topic1", &client2);
        broker.publish("topic1", Message::new("hello world"));
        assert_eq!(
            client1.write().unwrap().next_message().unwrap().content,
            "hello world"
        );
        assert_eq!(
            client2.write().unwrap().next_message().unwrap().content,
            "hello world"
        );
    }

    #[test]
    fn test_unsubscribe() {
        let broker = Broker::new();
        let client1 = Client::new();
        let client2 = Client::new();
        broker.subscribe("topic1", &client1);
        broker.subscribe("topic1", &client2);
        broker
            .unsubscribe("topic1", client1.read().unwrap().id())
            .unwrap();
        broker.publish("topic1", Message::new("hello world"));
        assert_eq!(client1.write().unwrap().next_message(), None);
        assert_eq!(
            client2.write().unwrap().next_message().unwrap().content,
            "hello world"
        );
    }

    #[test]
    fn test_multiple_topics() {
        let broker = Broker::new();
        let client = Client::new();
        broker.subscribe("topic1", &client);
        broker.subscribe("topic2", &client);
        broker.publish("topic1", Message::new("hello topic1"));
        broker.publish("topic2", Message::new("hello topic2"));
        assert_eq!(
            client.write().unwrap().next_message().unwrap().content,
            "hello topic1"
        );
        assert_eq!(
            client.write().unwrap().next_message().unwrap().content,
            "hello topic2"
        );
    }

    #[test]
    fn test_ring_buffer() {
        let broker = Broker::new();
        let client = Client::with_ring_buffer_size(2);
        broker.subscribe("topic1", &client);
        broker.publish("topic1", Message::new("message1"));
        broker.publish("topic1", Message::new("message2"));
        broker.publish("topic1", Message::new("message3"));
        assert_eq!(
            client.write().unwrap().next_message().unwrap().content,
            "message2"
        );
        assert_eq!(
            client.write().unwrap().next_message().unwrap().content,
            "message3"
        );
    }

    #[test]
    fn test_peek_message() {
        let broker = Broker::new();
        let client = Client::new();
        broker.subscribe("topic1", &client);
        broker.publish("topic1", Message::new("peek this"));

        assert_eq!(
            client.read().unwrap().peek_message().unwrap().content,
            "peek this"
        );

        assert_eq!(
            client.write().unwrap().next_message().unwrap().content,
            "peek this"
        );

        assert!(client.write().unwrap().next_message().is_none());
    }

    #[test]
    fn test_weak_reference_cleanup() {
        let broker = Broker::new();

        {
            let client = Client::new();
            broker.subscribe("topic1", &client);

            assert!(broker.subscribers.read().unwrap().contains_key("topic1"));

            broker.publish("topic1", Message::new("Test"));

            assert_eq!(
                client.write().unwrap().next_message().unwrap().content,
                "Test"
            );
        }

        if let Some(subscribers) = broker.subscribers.read().unwrap().get("topic1") {
            assert!(subscribers[0].upgrade().is_none());
        } else {
            panic!("Topic 'topic1' should still exist at this point.");
        }

        broker.publish("topic1", Message::new("Test 2"));

        assert!(!broker.subscribers.read().unwrap().contains_key("topic1"));
    }
}
