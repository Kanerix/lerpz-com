infra_services := "postgres dragonfly qdrant minio"
db_url := "postgres://lerpz:Password123@localhost:6432/primary"

export SQLX_OFFLINE := "true"

_default:
    @just --list

# Start infra, then every app service on the host (Ctrl-C stops them all)
[group('start')]
dev: infra
    #!/usr/bin/env bash
    set -euo pipefail
    trap 'kill 0' EXIT INT TERM
    cargo run -p api &
    cargo run -p artoo &
    bun --filter @lerpz/app dev &
    bun --filter @lerpz/www dev &
    wait

# Start only the third-party infra containers, and wait until they are healthy
[group('start')]
infra:
    docker compose up -d --wait {{ infra_services }}
    docker compose up -d minio-init

# Product UI on http://localhost:3001
[group('start')]
app:
    bun --filter @lerpz/app dev

# Company site on http://localhost:3000
[group('start')]
www:
    bun --filter @lerpz/www dev

# Core backend API on http://localhost:4000 (docs at /scalar)
[group('start')]
api:
    cargo run -p api

# AI agent on http://localhost:4001 (docs at /scalar)
[group('start')]
artoo:
    cargo run -p artoo

# Agent provisioner on http://localhost:5000 (requires a Kubernetes cluster)
[group('start')]
forge:
    cargo run -p forge

# Build and run the whole stack in containers (compose `full` profile)
[group('start')]
up:
    docker compose --profile full up -d --build

# Stop and remove every container, leaving volumes intact
[group('start')]
down:
    docker compose --profile full --profile k8s down

# Apply pending sqlx migrations
[group('db')]
migrate:
    DATABASE_URL={{ db_url }} cargo sqlx migrate run --source migrations

# Create a new timestamped migration
[group('db')]
migration NAME:
    DATABASE_URL={{ db_url }} cargo sqlx migrate add --source migrations {{ NAME }}

# Regenerate the offline .sqlx cache (needs postgres running and migrated)
[group('db')]
prepare:
    SQLX_OFFLINE=false DATABASE_URL={{ db_url }} cargo sqlx prepare --workspace

# Open a psql shell against the running postgres container
[group('db')]
psql:
    docker compose exec postgres psql -U lerpz -d primary

# Run every check: clippy, svelte-check and biome
[group('verify')]
check: check-rust check-ts

# Clippy across the Cargo workspace
[group('verify')]
check-rust:
    cargo clippy --workspace --all-targets

# svelte-check and biome across the Bun workspace
[group('verify')]
check-ts:
    bun run check
    bun run lint

# Format Rust and TypeScript/Svelte in place
[group('verify')]
fmt:
    cargo fmt --all
    bun run format

# Run the Cargo test suite
[group('verify')]
test:
    cargo test --workspace

# Run the criterion benchmarks
[group('verify')]
bench:
    cargo bench -p benchmarks

# Release-build every service
[group('verify')]
build:
    cargo build --workspace --release
    bun run build

# Install Bun workspace dependencies
[group('tools')]
install:
    bun install

# Regenerate the app's API client from the running api's OpenAPI spec
[group('tools')]
openapi:
    bun run generate:api

# Show container status
[group('tools')]
ps:
    docker compose --profile full --profile k8s ps

# Tail container logs, e.g. `just logs api`
[group('tools')]
logs *ARGS:
    docker compose --profile full --profile k8s logs -f {{ ARGS }}

# Delegate to k8s/justfile, e.g. `just k8s status`
[group('tools')]
k8s *ARGS:
    @just --justfile k8s/justfile --working-directory k8s {{ ARGS }}

# DESTRUCTIVE: stop everything and delete the data volumes
[confirm("Delete all data related to the local development environment? [y/N]")]
[group('tools')]
reset:
    docker compose --profile full --profile k8s down -v
