use std::sync::Arc;

use anyhow::{Ok, Result};
use async_trait::async_trait;
use domain::{
    entities::stub_domain_entity::KeyValue,
    ports::repositories::mockserver_http_service_port::MockserverHttpServicePort,
};

use super::mockserver_configuration::get_mockserver_base_url;

#[derive(Debug)]
pub struct MockserverHttpService {
    reqwest_client: Arc<reqwest::Client>,
}

impl MockserverHttpService {
    pub fn new(reqwest_client: Arc<reqwest::Client>) -> Self {
        Self { reqwest_client }
    }
}

#[async_trait]
impl MockserverHttpServicePort for MockserverHttpService {
    async fn execute_call(&self) -> Result<KeyValue> {
        let url = format!("{}/key-value", get_mockserver_base_url()?);

        let response = self.reqwest_client.post(url)
            .header("api-key", "key")
            .json(&serde_json::json!({ "id": "id" }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to execute call"));
        }

        let key_value: KeyValue = response.json::<KeyValue>().await?;

        Ok(key_value)
    }
}
