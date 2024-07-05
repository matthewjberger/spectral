use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EngineMessage {
    #[default]
    Empty,
}
