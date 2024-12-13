use aws_sdk_sqs as sqs;
use std::time::Duration;
use tokio::time::sleep;

#[::tokio::main]
async fn main() -> Result<(), sqs::Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_sqs::Client::new(&config);

    let queue_url = "http://sqs.us-east-1.localhost.localstack.cloud:4566/000000000000/rust-test-sqs-queue.fifo";

    loop {
        let receive_result = client.receive_message()
            .queue_url(queue_url)
            .max_number_of_messages(10)
            .wait_time_seconds(20)
            .send().await;

        match receive_result {
            Ok(output) => {
                if let Some(messages) = output.messages {
                    for message in messages {
                        if let Some(body) = message.body {
                            println!("Received message: {}", body);

                            // Process the message here

                            // Delete the message after processing
                            if let Some(receipt_handle) = message.receipt_handle {
                                client.delete_message()
                                    .queue_url(queue_url)
                                    .receipt_handle(receipt_handle)
                                    .send().await?;
                            }
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("Error receiving messages: {:?}", err);
            }
        }

        // Sleep for a short duration before polling again
        sleep(Duration::from_secs(5)).await;
    }
}