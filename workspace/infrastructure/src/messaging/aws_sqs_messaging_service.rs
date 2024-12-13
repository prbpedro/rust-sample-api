use std::sync::Arc;

use anyhow::{bail, Result};
use async_trait::async_trait;
use domain::ports::messaging::messaging_service_port::MessagingServicePort;

use tracing::{instrument, Level};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::log_with_span;
use crate::logging::logging_task_local::REQUEST_DATA;
use opentelemetry::trace::TraceContextExt;

#[derive(Debug)]
pub struct AwsSqsMessagingService {
    aws_client: Arc<aws_sdk_sqs::Client>,
    aws_sqs_queue_url: String,
}

impl AwsSqsMessagingService {
    pub fn new(aws_client: Arc<aws_sdk_sqs::Client>, aws_sqs_queue_url: String) -> Self {
        Self { aws_client , aws_sqs_queue_url}
    }
}

#[async_trait]
impl MessagingServicePort for AwsSqsMessagingService {
    #[instrument(skip_all, err)]
    async fn send_message(
        &self,
        partition_id: String,
        deduplication_id: String,
        body: String) -> Result<()> {
        let response = &self.aws_client
            .send_message()
            .queue_url(&self.aws_sqs_queue_url)
            .message_body(body)
            .message_group_id(partition_id)
            .message_deduplication_id(format!("CREATE#{}", deduplication_id))
            .send()
            .await;

        match response {
            Ok(_) => {
                log_with_span!(Level::INFO, "Message sent. Response={:?}", response);
                Ok(())
            },
            Err(err) => bail!("Error sending message: {:?}", err),
        }
    }
}
