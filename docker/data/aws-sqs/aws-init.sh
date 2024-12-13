# Delete existing queues if they exist
aws sqs delete-queue --queue-url http://localhost:4566/000000000000/rust-test-sqs-queue --profile localstack 2>/dev/null || true
aws sqs delete-queue --queue-url http://localhost:4566/000000000000/rust-test-sqs-dlq --profile localstack 2>/dev/null || true


# Create the Dead Letter Queue (DLQ)
aws sqs create-queue --queue-name rust-test-sqs-dlq --profile localstack

# Get the ARN of the DLQ
DLQ_ARN=$(aws sqs get-queue-attributes --queue-url http://localhost:4566/000000000000/rust-test-sqs-dlq --attribute-names QueueArn --query 'Attributes.QueueArn' --output text --profile localstack)

# Create the main queue with the RedrivePolicy attribute set to use the DLQ
aws sqs create-queue \
  --queue-name rust-test-sqs-queue.fifo \
  --attributes '{
    "FifoQueue":"true","ContentBasedDeduplication":"true",
    "RedrivePolicy": "{\"deadLetterTargetArn\":\"'$DLQ_ARN'\",\"maxReceiveCount\":\"5\"}"
  }' \
  --profile localstack