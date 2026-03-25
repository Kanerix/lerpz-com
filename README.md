# Lerpz Core

A monorepo containing shared libraries, services, and packages for the Lerpz
platform — an enterprise AI portal that provides chat interfaces, user
management, and organizational tools, all backed by Microsoft Entra ID
authentication.

## Tech Stack

| Layer        | Technology                                                |
| ------------ | --------------------------------------------------------- |
| **Frontend** | Next.js 16, React 19, Tailwind CSS 4, Radix UI, Zustand   |
| **Backend**  | Rust (nightly), Axum, Tokio                               |
| **Database** | PostgreSQL 17 (sqlx), Dragonfly/Redis (bb8)               |
| **Auth**     | Microsoft Entra ID (Azure AD), MSAL, JWT                  |
| **AI**       | OpenAI API (via async-openai)                             |
| **API Docs** | OpenAPI / Swagger UI (utoipa)                             |
| **Tooling**  | pnpm, Turborepo, Biome, Cargo workspaces, Docker, Traefik |

## Prerequisites

- [Rust](https://rustup.rs/) (nightly toolchain — managed via `rust-toolchain.toml`)
- [Node.js](https://nodejs.org/) >= 22
- [pnpm](https://pnpm.io/) 10.x
- [Docker](https://www.docker.com/) & Docker Compose
- [mkcert](https://github.com/FiloSottile/mkcert) (for local TLS certificates)

## Getting Started

### 1. Clone the repository

```sh
git clone https://github.com/lerpz/lerpz-core.git
cd lerpz-core
```

### 2. Install dependencies

```sh
# Rust (handled automatically by cargo)
cargo build

# Node
pnpm install
```

### 3. Set up environment files

Create `.env` files for each service based on their requirements:

- `svc/portal/.env` — Frontend environment variables (Entra ID tenant/client IDs, API URLs)
- `svc/portal-api/.env` — Backend environment variables (database URLs, Azure AD config, OpenAI keys)

For Docker-based development, the services use `.env.docker` variants.

## Local Development with Docker Compose

The Docker Compose setup includes PostgreSQL, Dragonfly (Redis-compatible), and
Traefik (reverse proxy with TLS). It supports two modes:

- **Default mode** — Only infrastructure services start (Postgres, Dragonfly,
  Traefik). Traefik routes `lerpz.local` and `api.lerpz.local` to apps running
  locally on your host machine.
- **Full mode** — All services run in containers, including the portal and
  portal-api.

### 1. Configure Microsoft Entra ID

- Register an app in Azure Entra ID.
- Select **ID tokens (used for implicit and hybrid flows)**.
- Add the following redirect URI:

```
https://lerpz.local/api/auth/callback/microsoft-entra-id
```

### 2. Generate TLS certificates

Use mkcert to create local certificates:

```sh
mkcert -cert-file certs/cert.pem -key-file certs/key.pem lerpz.local api.lerpz.local
```

### 3. Update your hosts file

Add these entries to `/etc/hosts`:

```
127.0.0.1 lerpz.local
127.0.0.1 api.lerpz.local
```

### 4. Start the containers

#### Default mode (local development)

Start only the infrastructure services. Traefik will proxy requests to apps
running on your host machine (`localhost:3000` and `localhost:3001`):

```sh
docker compose up
```

Then start your apps locally in separate terminals:

```sh
pnpm dev:portal              # Portal on localhost:3000
cargo run -p portal-api      # Portal API on localhost:3001
```

#### Full mode (everything containerized)

Start all services, including the portal and portal-api, inside containers:

```sh
docker compose --profile full up --build
```

### Accessing services

Once running, the services are available at:

| Service           | URL                                |
| ----------------- | ---------------------------------- |
| Portal (HTTPS)    | https://lerpz.local                |
| Portal API        | https://api.lerpz.local            |
| Swagger UI        | https://api.lerpz.local/swagger-ui |
| Traefik Dashboard | http://localhost:8080               |

## Development Scripts

### Frontend (Portal)

```sh
pnpm dev:portal        # Start dev server (Turbopack)
pnpm build:portal      # Production build
pnpm start:portal      # Start production server
pnpm lint:portal       # Lint with Biome
pnpm format:portal     # Format with Biome
```

### Backend (Portal API)

```sh
cargo run -p portal-api     # Run the API server
cargo test                  # Run all tests
cargo bench                 # Run benchmarks
```

## Crates

| Crate            | Description                                                        |
| ---------------- | ------------------------------------------------------------------ |
| `lerpz-axum`     | Axum middleware, error handling, graceful shutdown & Azure AD auth |
| `lerpz-jwt`      | JWT token generation and verification                              |
| `lerpz-macros`   | Procedural macros (e.g., `#[with_instance]` for Axum handlers)     |
| `lerpz-metadata` | MongoDB-backed content metadata models and client                  |
| `lerpz-model`    | Core domain models with serde and sqlx (Postgres) support          |
| `lerpz-pwd`      | Argon2 password hashing, verification & policy validation          |
| `lerpz-utils`    | Shared configuration, env utilities & UPN handling                 |

## Packages

| Package                    | Description                                                     |
| -------------------------- | --------------------------------------------------------------- |
| `@lerpz/ui`                | Shared React component library (Radix UI, Tailwind)             |
| `@lerpz/typescript-config` | Shared TypeScript config presets (base, Next.js, React library) |

## Database

The project uses PostgreSQL with migrations managed by sqlx. The schema includes
tables for conversations and messages supporting the AI chat feature. See
[`docs/DATABASE.md`](docs/DATABASE.md) for the full ER diagram.

Run migrations:

```sh
sqlx migrate run
```
