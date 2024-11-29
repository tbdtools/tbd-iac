use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait Provider {
    async fn create_resource(&self, resource: ResourceRequest) -> Result<ResourceResponse>;
    async fn delete_resource(&self, id: String) -> Result<()>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceRequest {
    pub provider: String,
    pub resource_type: String,
    pub properties: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceResponse {
    pub id: String,
    pub state: serde_json::Value,
}
