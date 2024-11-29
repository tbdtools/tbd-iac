use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Deserialize)]
pub struct Resource {
    id: String,
    provider: String,
    state: serde_json::Value,
}

pub struct StateManager {
    resources: Arc<RwLock<Vec<Resource>>>,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            resources: Arc::new(RwLock::new(Vec::new())),
        }
    }
}
