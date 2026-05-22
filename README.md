# Lerpz

A monorepo containing shared libraries, services, and packages for the Lerpz
platform — an internal enterprise AI portal that provides chat interfaces, user
management, and organizational tools, all backed by Microsoft Entra ID
authentication.

## Services

| Name | Role | Stack |
|---|---|---|
| `holonet` | Frontend | Next.js |
| `venator` | Backend API | Rust / Axum |
| `artoo` | AI Agent | Rust |
| `ilum` | Vector database | Qdrant |
| `kamino` | Primary database | PostgreSQL |
| `naboo` | Cache | Dragonfly (Redis-compatible) |
| `geonosis` | Object storage | MinIO |

Service names follow a Star Wars Republic era theme. `holonet` and `venator` are directly linked — the Venator-class Star Destroyer was equipped with a HoloNet transceiver, making it the engine behind the network.
[docs/NAMING.md](docs/NAMING.md) for the full rationale behind each name.

## Prerequisites

- [Rust](https://rustup.rs/) (edition 2024)
- [Node.js](https://nodejs.org/) >= 22
- [pnpm](https://pnpm.io/) 10.x
- [Docker](https://www.docker.com/) & Docker Compose

### 1. Configure Microsoft Entra ID

- Register an app in Azure Entra ID.
- Select **ID tokens (used for implicit and hybrid flows)**.
- Add the following redirect URI:

```bash
http://localhost:3000/api/auth/callback/microsoft-entra-id
# or
https://api.lerpz.local/api/auth/callback/microsoft-entra-id
```

#### 1.1. Generate TLS certificates (for traefik)

Use mkcert to create local certificates:

```sh
mkcert -cert-file certs/cert.pem -key-file certs/key.pem lerpz.local api.lerpz.local
```

#### 1.2. Update your hosts file (for traefik)

Add these entries to `/etc/hosts`:

```
127.0.0.1 lerpz.local api.lerpz.local agent.lerpz.local
```

### 2. Start the containers

#### Default mode (local development)

Start only the infrastructure services. If you followed the Traefik
steps, requests will be proxied to apps running on your local machine
(`localhost:3000` and `localhost:3001`):

```sh
docker compose up
```

#### Full mode (everything containerized)

Start all services using Docker:

```sh
docker compose --profile full up --build
```

## Database

The project uses PostgreSQL with migrations managed by
[sqlx](https://github.com/launchbadge/sqlx). The schema includes tables for
conversations and messages supporting the AI chat feature.

Run migrations:

```sh
cargo sqlx migrate run
```
