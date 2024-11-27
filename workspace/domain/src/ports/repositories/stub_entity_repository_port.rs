use crate::entities::stub_domain_entity::StubEntity;
use anyhow::Result;
use async_trait::async_trait;

use super::transaction_port::TransactionPort;

#[async_trait]
pub trait StubEntityRepositoryPort: Send + Sync {
    async fn add(&self, entity: &StubEntity) -> Result<StubEntity>;
    async fn get(&self, id: i32) -> Result<Option<StubEntity>>;
    async fn get_within_transaction(&self, id: i32, txn: &Box<dyn TransactionPort>) -> Result<Option<StubEntity>>;
    async fn update_within_transaction(&self, entity: &StubEntity, txn: &Box<dyn TransactionPort>) -> Result<StubEntity>;
    async fn get_all(&self) -> Result<Vec<StubEntity>>;
}
