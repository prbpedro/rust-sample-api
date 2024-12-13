# rust-api

## Building

Building:
```bash
cd workspace/
cargo build
```
## Running

Running:
```bash
cd docker/
docker compose up -d
cd ../workspace/
RUST_LOG=info,sqlx::postgres::notice=warn,sqlx_core=warn,sea_orm_migration::migrator=warn \
RUST_BACKTRACE=1 \
DATABASE_CONNECTION_STRING=postgres://postgres:password@localhost:5432/rust-sample-db \
HTTP_REQUEST_METRICS_EXPONENTIAL_SECONDS=0.5,0.8,1 \
MOCKSERVER_BASE_URL=http://localhost:1080 \
cargo run
```
## RabbitMQ

aws sqs list-queues --profile localstack

https://docs.aws.amazon.com/sdk-for-rust/latest/dg/rust_sqs_code_examples.html