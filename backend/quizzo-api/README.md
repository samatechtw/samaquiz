# Quizzo API

The Quizzo API is written in Rust. Use the instructions below to install and run.

## Environment

Environment variables can be provided to `cargo run`, or in a `.env` file in this directory.

Default values can be found in the Dockerfile.

## Prerequisites

**Install Rust**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**SQLx CLI**

```bash
cargo install sqlx-cli --features postgres
```

**Migrations**

```bash
# Create metadata db and run migrations
# This is handled automatically when the app starts
sqlx db create
sqlx migrate --source migrations run
```

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
docker build -t quizzo-api -f backend/quizzo-api/Dockerfile --target=dev .

docker run -p 3000:3000 quizzo-api
```

## Usage

Runs on port 3000 by default.

```bash
# Health check
curl 'http://localhost:3000/api/healthz'
```
