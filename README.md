# Lerpz

A monorepo containing shared libraries, services, and packages for the Lerpz
platform — an internal enterprise AI portal that provides chat interfaces, user
management, and organizational tools, all backed by Microsoft Entra ID
authentication.

## Services

| Name | Role | Stack | Port |
|---|---|---|---|
| `www` | Company / landing site | SvelteKit (static) | 3000 |
| `app` | Product UI | SvelteKit | 3001 |
| `api` | Backend API | Rust / Axum | 4000 |
| `artoo` | AI agent | Rust | 4001 |
| `forge` | Agent infrastructure provisioner | Rust / Axum / kube | 5000 |
| `qdrant` | Vector database | Qdrant | 6333 / 6334 |
| `postgres` | Primary database | PostgreSQL | 6432 |
| `dragonfly` | Cache | Dragonfly (Redis-compatible) | 6379 |
| `minio` | Object storage | MinIO | 6000 / 6001 |

Ports 3000–3999 are web pages, 4000–4999 are publicly reachable APIs, 5000–5999
are internal services, and 6000–6999 is third-party infrastructure. The
infrastructure ports above are what you connect to **from your machine**;
in-network those containers keep their vendor defaults (`postgres:5432`,
`minio:9000`). See [docs/NAMING.md](docs/NAMING.md) for the naming rules and the
conventions for adding a new service.

## Prerequisites

- [Rust](https://rustup.rs/) (edition 2024)
- [Bun](https://bun.sh/) >= 1.4
- [Docker](https://www.docker.com/) & Docker Compose

### 1. Configure Microsoft Entra ID

- Register an app in Azure Entra ID.
- Select **ID tokens (used for implicit and hybrid flows)**.
- Add the following redirect URI:

```bash
http://localhost:3001/api/auth/callback/microsoft-entra-id
# or
https://app.lerpz.local/api/auth/callback/microsoft-entra-id
```

#### 1.1. Generate TLS certificates (for traefik)

Use mkcert to create local certificates:

```sh
mkcert -cert-file certs/cert.pem -key-file certs/key.pem \
  lerpz.local www.lerpz.local app.lerpz.local api.lerpz.local agent.lerpz.local
```

#### 1.2. Update your hosts file (for traefik)

Add these entries to `/etc/hosts`:

```
127.0.0.1 lerpz.local www.lerpz.local app.lerpz.local api.lerpz.local agent.lerpz.local
```

### 2. Start the containers

#### Default mode (local development)

Start only the infrastructure services. If you followed the Traefik
steps, requests will be proxied to apps running on your local machine
(`localhost:3001` for `app`, `localhost:4000` for `api`):

```sh
docker compose up
```

#### Full mode (everything containerized)

Start all services using Docker:

```sh
docker compose --profile full up --build
```

#### Forge

`forge` provisions agent infrastructure through the Kubernetes API, so it has no
self-contained local mode and sits behind its own profile:

```sh
docker compose --profile k8s up --build forge
```

It mounts `~/.kube` read-only and exits on startup if no cluster answers. Because
a kind/minikube kubeconfig points at `127.0.0.1` — which inside the container is
the container itself — running `forge` in the kind cluster under [`k8s/`](k8s)
is usually the better path. See [k8s/README.md](k8s/README.md).

## Database

The project uses PostgreSQL with migrations managed by
[sqlx](https://github.com/launchbadge/sqlx). The schema includes tables for
conversations and messages supporting the AI chat feature.

Run migrations:

```sh
cargo sqlx migrate run
```
