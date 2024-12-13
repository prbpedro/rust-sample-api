use core::fmt;
use std::{fmt::format, sync::Arc};

use anyhow::Result;
use domain::{
    entities::stub_domain_entity::StubEntity,
    ports::{
        messaging::messaging_service_port::MessagingServicePort,
        repositories::{
            mockserver_http_service_port::MockserverHttpServicePort,
            stub_entity_repository_port::StubEntityRepositoryPort,
            transaction_port::TransactionPort,
        },
    },
};

pub struct StubEntityUseCase {
    repository: Arc<dyn StubEntityRepositoryPort>,
    mockserver_http_service: Arc<dyn MockserverHttpServicePort>,
    messaging_service: Arc<dyn MessagingServicePort>,
}

impl StubEntityUseCase {
    pub fn new(
        repository: Arc<dyn StubEntityRepositoryPort>,
        mockserver_http_service: Arc<dyn MockserverHttpServicePort>,
        messaging_service: Arc<dyn MessagingServicePort>,
    ) -> Self {
        Self {
            repository,
            mockserver_http_service,
            messaging_service,
        }
    }

    pub async fn list(&self) -> Result<Vec<StubEntity>> {
        self.repository.get_all().await
    }

    pub async fn add(&self, entity: &mut StubEntity) -> Result<StubEntity> {
        let key_value = self.mockserver_http_service.execute_call().await?;
        entity.value = key_value;
        let entity = self.repository.add(entity).await?;
        self.messaging_service
            .send_message(
                entity.id.unwrap().to_string(),
                format!("{}${}${}", "created", "stub-entity", entity.id.unwrap()),
                serde_json::to_string(&entity)?,
            )
            .await?;
        Ok(entity)
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
