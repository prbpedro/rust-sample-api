use std::{sync::Arc, time::Duration};

use anyhow::Result;
use async_trait::async_trait;
use domain::ports::repositories::transaction_port::TransactionPort;
use sea_orm::{ConnectOptions, Database, TransactionTrait};

use super::database_data::{DatabaseConnection, Transaction};

//TODO: dotenv
const DATABASE_URL: &str = "postgres://postgres:password@localhost:5432/rust-sample-db";

impl DatabaseConnection<sea_orm::DatabaseConnection> {
    pub async fn new() -> Result<Arc<DatabaseConnection<sea_orm::DatabaseConnection>>, anyhow::Error>
    {
        let mut opt = ConnectOptions::new(DATABASE_URL.to_owned());
        opt.max_connections(10)
            .min_connections(2)
            .connect_timeout(Duration::from_secs(2))
            .idle_timeout(Duration::from_secs(60))
            .max_lifetime(Duration::from_secs(20))
            .sqlx_logging(false);
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
        self.txn
            .commit()
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))
    }

    async fn rollback(self: Box<Self>) -> Result<()> {
        self.txn
            .rollback()
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))
    }
}
