use anyhow::Result;
use domain::ports::repositories::{mockserver_http_service_port::MockserverHttpServicePort, stub_entity_repository_port::StubEntityRepositoryPort};
use infrastructure::{database::repositories::{database_data::DatabaseConnection, stub_entity_sea_orm_postgres_repository::StubEntitySeaOrmPostgresRepository}, http::mockserver::{mockserver_configuration::get_mockserver_base_url, mockserver_http_service::MockserverHttpService}};
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

        let mockserver_http_service = build_mock_server_http_service();

        let stub_entity_use_case = build_stub_entity_use_case(
            &stub_entity_repository,
            &mockserver_http_service);

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

fn build_mock_server_http_service() -> Arc<dyn MockserverHttpServicePort> {
    let reqwest_client = Arc::new(reqwest::Client::new());
    let base_url = get_mockserver_base_url().unwrap();
    Arc::new(MockserverHttpService::new(reqwest_client, base_url))
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
    mockserver_http_service: &Arc<dyn MockserverHttpServicePort>,
) -> Arc<StubEntityUseCase> {
    Arc::new(StubEntityUseCase::new(repository.clone(), mockserver_http_service.clone()))
}
