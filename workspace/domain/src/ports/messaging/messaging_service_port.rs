use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait MessagingServicePort : Send + Sync {
    async fn send_message(
        &self, 
        partition_id: String,
        deduplication_id: String,
        body: String) -> Result<()>;
}