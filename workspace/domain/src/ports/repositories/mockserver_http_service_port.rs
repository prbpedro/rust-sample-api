use crate::entities::stub_domain_entity::KeyValue;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait MockserverHttpServicePort: Send + Sync {
    async fn execute_call(&self) -> Result<KeyValue>;
}
