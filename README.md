# todo_api

A little REST API tool for todo list based on PostgreQL for myself using it with telegram bot and cli tool.

## Quick Start

### 1. Dependencies

- Rust (edition 2024)
- Docker or Podman
- sqlx-cli

### 2. Clone and configure

```bash
git clone https://github.com/matgo1/todo_api.git
cd todo_api
cp .env.example .env
```

! Don't forget to change url on yours

### 3. Start Postgres

- `docker compose up -d postgre`

### 4. Start migrations

- sqlx migrate run

### 5. Run the API

```bash
cargo build --release
./target/release/todo_cli
```
