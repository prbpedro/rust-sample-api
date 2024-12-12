use std::sync::Arc;

use crate::database::entities::stub_database_entity::*;
use anyhow::{bail, Result};
use async_trait::async_trait;
use domain::{
    entities::stub_domain_entity::StubEntity,
    ports::repositories::{
        stub_entity_repository_port::StubEntityRepositoryPort, transaction_port::TransactionPort,
    },
};
use sea_orm::{ActiveModelTrait, DatabaseTransaction, EntityTrait};

use super::database_data::{DatabaseConnection, Transaction};

#[derive(Debug)]
pub struct StubEntitySeaOrmPostgresRepository {
    db: Arc<DatabaseConnection<sea_orm::DatabaseConnection>>,
}

impl StubEntitySeaOrmPostgresRepository {
    pub fn new(db: Arc<DatabaseConnection<sea_orm::DatabaseConnection>>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl StubEntityRepositoryPort for StubEntitySeaOrmPostgresRepository {
    #[tracing::instrument(skip_all, err)]
    async fn add(&self, entity: &StubEntity) -> Result<StubEntity> {
        let active_model: ActiveModel = ActiveModel::from_domain(entity, false);

        let inserted_entity = active_model.insert(&self.db.conn).await;

        match inserted_entity {
            Ok(inserted_entity) => Ok(inserted_entity.to_domain()),
            Err(err) => bail!(err),
        }
    }

    #[tracing::instrument(skip_all, err)]
    async fn get(&self, id: i32) -> Result<Option<StubEntity>> {
        let entity = Entity::find_by_id(id).one(&self.db.conn).await;

        match entity {
            Ok(Some(entity)) => Ok(Some(entity.to_domain())),
            Ok(None) => Ok(None),
            Err(err) => bail!(err),
        }
    }

    #[tracing::instrument(skip_all, err)]
    async fn get_within_transaction(
        &self,
        id: i32,
        txn: &Box<dyn TransactionPort>,
    ) -> Result<Option<StubEntity>> {
        let entity = Entity::find_by_id(id)
            .one(
                &txn.as_any()
                    .downcast_ref::<Transaction<DatabaseTransaction>>()
                    .unwrap()
                    .txn,
            )
            .await;

        match entity {
            Ok(Some(entity)) => Ok(Some(entity.to_domain())),
            Ok(None) => Ok(None),
            Err(err) => bail!(err),
        }
    }

    #[tracing::instrument(skip_all, err)]
    async fn update_within_transaction(
        &self,
        entity: &StubEntity,
        txn: &Box<dyn TransactionPort>,
    ) -> Result<StubEntity> {
        let active_model: ActiveModel = ActiveModel::from_domain(entity, true);

        let updated_entity = active_model
            .update(
                &txn.as_any()
                    .downcast_ref::<Transaction<DatabaseTransaction>>()
                    .unwrap()
                    .txn,
            )
            .await;

        match updated_entity {
            Ok(inserted_entity) => Ok(inserted_entity.to_domain()),
            Err(err) => bail!(err),
        }
    }

    #[tracing::instrument(skip_all, err)]
    async fn get_all(&self) -> Result<Vec<StubEntity>> {
        let entities = Entity::find().all(&self.db.conn).await;
        match entities {
            Ok(entities) => Ok(entities.into_iter().map(|e| e.to_domain()).collect()),
            Err(err) => bail!(err),
        }
    }
}
