use std::any::Any;

use anyhow::Result;
use async_trait::async_trait;


#[async_trait]
pub trait TransactionPort: Any + Send + Sync {
    async fn commit(self: Box<Self>) -> Result<()>;
    async fn rollback(self: Box<Self>) -> Result<()>;
    fn as_any(&self) -> &dyn Any;
}
