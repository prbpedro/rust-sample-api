use core::fmt;
use std::sync::Arc;

use anyhow::Result;
use domain::{
    entities::stub_domain_entity::StubEntity,
    ports::repositories::{
        stub_entity_repository_port::StubEntityRepositoryPort, transaction_port::TransactionPort,
    },
};

pub struct StubEntityUseCase {
    repository: Arc<dyn StubEntityRepositoryPort>,
}

impl StubEntityUseCase {
    pub fn new(repository: Arc<dyn StubEntityRepositoryPort>) -> Self {
        Self { repository }
    }

    pub async fn list(&self) -> Result<Vec<StubEntity>> {
        self.repository.get_all().await
    }

    pub async fn add(&self, entity: &StubEntity) -> Result<StubEntity> {
        self.repository.add(entity).await
    }

    pub async fn update(
        &self,
        entity: &StubEntity,
        txn: &Box<dyn TransactionPort>,
    ) -> Result<StubEntity> {
        self.repository.update_within_transaction(entity, txn).await
    }

    pub async fn get(
        &self,
        id: i32,
        txn: Option<&Box<dyn TransactionPort>>,
    ) -> Result<Option<StubEntity>> {
        match txn {
            Some(txn) => self.repository.get_within_transaction(id, txn).await,
            None => self.repository.get(id).await,
        }
    }
}

impl fmt::Debug for StubEntityUseCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StubEntityUseCase").finish()
    }
}
