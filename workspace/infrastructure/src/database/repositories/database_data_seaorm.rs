use std::{sync::Arc, time::Duration};

use anyhow::{bail, Result};
use async_trait::async_trait;
use domain::ports::repositories::transaction_port::TransactionPort;
use sea_orm::{ConnectOptions, Database, TransactionTrait};

use crate::database::postgres_database_configuration::*;

use super::database_data::{DatabaseConnection, Transaction};

impl DatabaseConnection<sea_orm::DatabaseConnection> {
    pub async fn new() -> Result<Arc<DatabaseConnection<sea_orm::DatabaseConnection>>> {
        let mut opt = ConnectOptions::new(get_db_connection_string().unwrap());
        opt.max_connections(get_db_max_connections().unwrap())
            .min_connections(get_db_min_connections().unwrap())
            .connect_timeout(Duration::from_secs(
                get_db_connect_timeout_seconds().unwrap(),
            ))
            .idle_timeout(Duration::from_secs(
                get_db_idle_connection_timeout_seconds().unwrap(),
            ))
            .max_lifetime(Duration::from_secs(
                get_db_max_lifetime_connection_seconds().unwrap(),
            ))
            .sqlx_logging(get_db_sqlx_logging().unwrap());
        let db_connection = Database::connect(opt).await?;
        Ok(Arc::new(Self {
            conn: db_connection,
        }))
    }
}

impl Transaction<sea_orm::DatabaseTransaction> {
    pub async fn begin(
        db: &Arc<DatabaseConnection<sea_orm::DatabaseConnection>>,
    ) -> Result<Box<dyn TransactionPort>> {
        let txn = db.conn.begin().await?;
        Ok(Box::new(Self { txn }))
    }
}

#[async_trait]
impl TransactionPort for Transaction<sea_orm::DatabaseTransaction> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    async fn commit(self: Box<Self>) -> Result<()> {
        match self.txn.commit().await {
            Ok(_) => Ok(()),
            Err(e) => bail!(e),
        }
    }

    async fn rollback(self: Box<Self>) -> Result<()> {
        match self.txn.rollback().await {
            Ok(_) => Ok(()),
            Err(e) => bail!(e),
        }
    }
}
