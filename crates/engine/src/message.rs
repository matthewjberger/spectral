use std::sync::Arc;

use enum2contract::EnumContract;
use serde::{Deserialize, Serialize};

#[derive(EnumContract, Clone)]
pub enum EngineMessage {
    #[topic("empty")]
    Empty,
}

// pub type EngineBroker = crate::broker::Broker<EngineMessage>;
pub type EngineBrokerHandle = Arc<crate::broker::Broker<EngineMessage>>;

pub type EngineClient = crate::broker::Client<EngineMessage>;
pub type EngineClientHandle = crate::broker::ClientHandle<EngineMessage>;

/// Listens for and resolves engine messages
pub async fn message_task(broker: EngineBrokerHandle) {
    let mut client = EngineClient::new();
    broker.subscribe(&EngineMessage::empty_topic(), &mut client);
    loop {
        if let Ok(mut client) = client.write() {
            while let Some(message) = client.next_message() {
                match message {
                    EngineMessage::Empty => {
                        log::info!("Empty message received");
                    }
                }
            }
        }
    }
}
