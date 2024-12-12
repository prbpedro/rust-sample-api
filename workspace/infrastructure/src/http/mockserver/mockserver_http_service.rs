use std::sync::Arc;

use anyhow::{bail, Result};
use async_trait::async_trait;
use domain::{
    entities::stub_domain_entity::KeyValue,
    ports::repositories::mockserver_http_service_port::MockserverHttpServicePort,
};
use tracing::instrument;

#[derive(Debug)]
pub struct MockserverHttpService {
    reqwest_client: Arc<reqwest::Client>,
    base_url: String,
}

impl MockserverHttpService {
    pub fn new(reqwest_client: Arc<reqwest::Client>, base_url: String) -> Self {
        Self { reqwest_client, base_url }
    }
}

#[async_trait]
impl MockserverHttpServicePort for MockserverHttpService {

    #[instrument(skip_all, err)]
    async fn execute_call(&self) -> Result<KeyValue> {
        let url = format!("{}/key-value", self.base_url);

        let response = self.reqwest_client.post(url)
            .header("api-key", "key")
            .json(&serde_json::json!({ "id": "id" }))
            .send()
            .await?;

        if !response.status().is_success() {
            bail!("Failed to execute call. Response={:?}", response);
        }

        let key_value: KeyValue = response.json::<KeyValue>().await?;

        anyhow::Ok(key_value)
    }
}
