// tbd-iac/src/state/mod.rs
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resource {
    pub id: String,
    pub provider: String,
    pub state: serde_json::Value,
}

impl Resource {
    /// Creates a new resource with the given ID, provider, and state
    pub fn new(
        id: impl Into<String>,
        provider: impl Into<String>,
        state: serde_json::Value,
    ) -> Self {
        Self {
            id: id.into(),
            provider: provider.into(),
            state,
        }
    }

    /// Get the resource ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the provider name
    pub fn provider(&self) -> &str {
        &self.provider
    }

    /// Get the resource state
    pub fn state(&self) -> &serde_json::Value {
        &self.state
    }
}

#[derive(Default)]
pub struct StateManager {
    resources: Arc<RwLock<Vec<Resource>>>,
}

impl StateManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn add_resource(&self, resource: Resource) -> Result<()> {
        let mut resources = self.resources.write().await;
        resources.push(resource);
        Ok(())
    }

    pub async fn get_resource(&self, id: &str) -> Option<Resource> {
        let resources = self.resources.read().await;
        resources.iter().find(|r| r.id == id).cloned()
    }

    pub async fn list_resources(&self) -> Vec<Resource> {
        let resources = self.resources.read().await;
        resources.clone()
    }

    pub async fn remove_resource(&self, id: &str) -> Result<()> {
        let mut resources = self.resources.write().await;
        if let Some(index) = resources.iter().position(|r| r.id == id) {
            resources.remove(index);
        }
        Ok(())
    }
}
