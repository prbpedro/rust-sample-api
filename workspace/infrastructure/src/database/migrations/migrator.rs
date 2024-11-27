use anyhow::{bail, Result};
use sea_orm::DatabaseConnection;
use sea_orm_migration::prelude::*;

use super::m20241126_000001_create_stub_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20241126_000001_create_stub_table::Migration)]
    }
}

impl Migrator {
    pub async fn run_migrations(db: &DatabaseConnection) -> Result<()> {

        let migrator_result = Migrator::up(db, None).await;
        match migrator_result {
            Ok(_) => Ok(()),
            Err(err) => bail!(err),
        }
    }
}
