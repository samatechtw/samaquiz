# API Test Helper

The Quizzo API is written in Rust. Use the instructions below to install and run.

## Environment

Environment variables can be provided to `cargo run`, or in a `.env` file in this directory.

Default values can be found in the Dockerfile.

## Run

```bash
# Debug mode
cargo run

# Release mode
cargo run --release
```

## Build

```bash
cargo build --release
```

**Docker**

```bash
docker build -t api-test-helper -f backend/api-test-helper/Dockerfile --target=dev .

docker run -p 3001:3001 api-test-helper
```

## Usage

Runs on port 3001 by default.

A [requests.http](./requests.http) file is provided for testing the API. Install the VSCode extension "REST Client" to send the requests.

```bash
# Reset Platform DB
curl 'http://localhost:3001/actions/reset/db-app'
```
