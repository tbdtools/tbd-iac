use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resource {
    id: String,
    provider: String,
    state: serde_json::Value,
}

#[derive(Default)]
pub struct StateManager {
    resources: Arc<RwLock<Vec<Resource>>>,
}

impl StateManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a new resource to the state
    pub async fn add_resource(&self, resource: Resource) -> Result<()> {
        let mut resources = self.resources.write().await;
        resources.push(resource);
        Ok(())
    }

    /// Get a resource by ID
    pub async fn get_resource(&self, id: &str) -> Option<Resource> {
        let resources = self.resources.read().await;
        resources.iter().find(|r| r.id == id).cloned()
    }

    /// List all resources
    pub async fn list_resources(&self) -> Vec<Resource> {
        let resources = self.resources.read().await;
        resources.clone()
    }

    /// Remove a resource by ID
    pub async fn remove_resource(&self, id: &str) -> Result<()> {
        let mut resources = self.resources.write().await;
        if let Some(index) = resources.iter().position(|r| r.id == id) {
            resources.remove(index);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_add_and_get_resource() {
        let state = StateManager::new();
        let resource = Resource {
            id: "test-1".to_string(),
            provider: "aws".to_string(),
            state: json!({
                "type": "aws_vpc",
                "id": "vpc-123",
            }),
        };

        state.add_resource(resource.clone()).await.unwrap();
        let retrieved = state.get_resource("test-1").await.unwrap();
        assert_eq!(retrieved.id, "test-1");
        assert_eq!(retrieved.provider, "aws");
    }

    #[tokio::test]
    async fn test_list_resources() {
        let state = StateManager::new();
        let resource1 = Resource {
            id: "test-1".to_string(),
            provider: "aws".to_string(),
            state: json!({"type": "aws_vpc"}),
        };
        let resource2 = Resource {
            id: "test-2".to_string(),
            provider: "aws".to_string(),
            state: json!({"type": "aws_subnet"}),
        };

        state.add_resource(resource1).await.unwrap();
        state.add_resource(resource2).await.unwrap();

        let resources = state.list_resources().await;
        assert_eq!(resources.len(), 2);
    }

    #[tokio::test]
    async fn test_remove_resource() {
        let state = StateManager::new();
        let resource = Resource {
            id: "test-1".to_string(),
            provider: "aws".to_string(),
            state: json!({"type": "aws_vpc"}),
        };

        state.add_resource(resource).await.unwrap();
        assert!(state.get_resource("test-1").await.is_some());

        state.remove_resource("test-1").await.unwrap();
        assert!(state.get_resource("test-1").await.is_none());
    }
}
