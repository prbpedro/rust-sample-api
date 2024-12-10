use anyhow::Result;
use domain::ports::repositories::stub_entity_repository_port::StubEntityRepositoryPort;
use infrastructure::database::repositories::{database_data::DatabaseConnection, stub_entity_sea_orm_postgres_repository::StubEntitySeaOrmPostgresRepository};
use std::sync::Arc;

use crate::{
    services::stub_entity_update_service::StubEntityUpdateService,
    use_cases::stub_entity_use_case::StubEntityUseCase,
};

pub struct AppState {
    pub database_connection: Arc<DatabaseConnection<sea_orm::DatabaseConnection>>,
    pub stub_entity_use_case: Arc<StubEntityUseCase>,
    pub stub_entity_update_service: Arc<StubEntityUpdateService>,
}

impl AppState {
    pub async fn new() -> Result<Arc<AppState>> {
        let database_connection = DatabaseConnection::new().await?;

        let stub_entity_repository = build_stub_entity_repository(&database_connection);

        let stub_entity_use_case = build_stub_entity_use_case(&stub_entity_repository);

        let stub_entity_update_service = build_stub_entity_update_service(
            &stub_entity_use_case,
            &database_connection,
        );

        let app_state = Self {
            database_connection,
            stub_entity_use_case,
            stub_entity_update_service,
        };

        Ok(Arc::new(app_state))
    }
}

fn build_stub_entity_repository(
    database_connection: &Arc<DatabaseConnection<sea_orm::DatabaseConnection>>,
) -> Arc<dyn StubEntityRepositoryPort> {
    Arc::new(StubEntitySeaOrmPostgresRepository::new(
        database_connection.clone(),
    ))
}

fn build_stub_entity_update_service(
    stub_entity_use_case: &Arc<StubEntityUseCase>,
    database_connection: &Arc<DatabaseConnection<sea_orm::DatabaseConnection>>,
) -> Arc<StubEntityUpdateService> {
    Arc::new(StubEntityUpdateService::new(
        stub_entity_use_case.clone(),
        database_connection.clone(),
    ))
}

fn build_stub_entity_use_case(
    repository: &Arc<dyn StubEntityRepositoryPort>,
) -> Arc<StubEntityUseCase> {
    Arc::new(StubEntityUseCase::new(repository.clone()))
}
