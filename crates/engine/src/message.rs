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
// pub type EngineClientHandle = Arc<crate::broker::Client<EngineMessage>>;

/// Listens for and resolves engine messages
pub async fn message_task(broker: EngineBrokerHandle) {
    let client = EngineClient::new(broker.clone());
    client.subscribe(&EngineMessage::empty_topic()).await;
    loop {
        while let Some(message) = client.next().await {
            match message {
                EngineMessage::Empty => {
                    log::info!("Empty message received");
                }
            }
        }
    }
}
