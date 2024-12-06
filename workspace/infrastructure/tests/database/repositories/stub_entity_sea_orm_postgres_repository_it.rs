use std::env;
use std::sync::Arc;

use domain::entities::stub_domain_entity::{KeyValue, StubEntity};
use domain::ports::repositories::stub_entity_repository_port::StubEntityRepositoryPort;
use infrastructure::database::repositories::database_data::DatabaseConnection;
use infrastructure::database::repositories::stub_entity_sea_orm_postgres_repository::StubEntitySeaOrmPostgresRepository;
use tokio;

async fn setup_db() -> Arc<DatabaseConnection<sea_orm::DatabaseConnection>> {
    env::set_var(
        "DATABASE_CONNECTION_STRING",
        "postgres://postgres:password@localhost:5432/rust-sample-db",
    );

    let db_connection = DatabaseConnection::new().await.unwrap();

    infrastructure::database::migrations::migrator::Migrator::run_migrations(&db_connection.conn)
        .await
        .unwrap();

    db_connection
}

#[tokio::test]
async fn test_add_and_get_stub_entity() {
    let db = setup_db().await;
    let repository = StubEntitySeaOrmPostgresRepository::new(db);

    let stub_entity = StubEntity {
        id: None,
        name: "Test Entity".to_string(),
        value: KeyValue {
            id: 1,
            name: "Test Value".to_string(),
        },
        auto_ref: Some(1),
    };

    // Test add
    let inserted_entity = repository.add(&stub_entity).await.unwrap();
    assert_eq!(inserted_entity.name, "Test Entity");

    // Test get
    let fetched_entity = repository.get(inserted_entity.id.unwrap()).await.unwrap();
    assert_eq!(fetched_entity.unwrap().name, "Test Entity");
}
