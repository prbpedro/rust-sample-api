use std::sync::Arc;

use anyhow::Result;
use domain::entities::stub_domain_entity::StubEntity;
use infrastructure::database::repositories::database_data::{DatabaseConnection, Transaction};
use tracing::instrument;

use crate::{
    handlers::dtos::stub_entity_dtos::StubEntityUpdateDto,
    use_cases::stub_entity_use_case::StubEntityUseCase,
};

#[derive(Debug)]
pub struct StubEntityUpdateService {
    stub_entity_use_case: Arc<StubEntityUseCase>,
    database_connection: Arc<DatabaseConnection<sea_orm::DatabaseConnection>>,
}

impl StubEntityUpdateService {
    pub fn new(
        stub_entity_use_case: Arc<StubEntityUseCase>,
        database_connection: Arc<DatabaseConnection<sea_orm::DatabaseConnection>>,
    ) -> Self {
        Self {
            stub_entity_use_case,
            database_connection,
        }
    }

    #[instrument(skip(self, id, dto), err)]
    pub async fn update(&self, id: i32, dto: StubEntityUpdateDto) -> Result<Option<StubEntity>> {
        let txn = Transaction::begin(&self.database_connection).await?;

        let entity = self.stub_entity_use_case.get(id, Some(&txn)).await?;

        match entity {
            Some(mut entity) => {
                if dto.name.is_some() {
                    entity.name = dto.name.unwrap().to_string();
                }

                if dto.value.is_some() {
                    entity.value = dto.value.unwrap().to_domain();
                }

                if dto.auto_ref.is_some() {
                    entity.auto_ref = dto.auto_ref;
                }

                match self.stub_entity_use_case.update(&entity, &txn).await {
                    Ok(_) => {
                        txn.commit().await?;
                        Ok(Some(entity))
                    }
                    Err(e) => {
                        txn.rollback().await?;
                        Err(e)
                    }
                }
            }
            None => {
                txn.rollback().await?;
                Ok(None)
            }
        }
    }
}
