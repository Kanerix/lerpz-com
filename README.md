<img width="100%" src="./banner.png" alt="Lerpz">

<p align="center">
    <a href="https://github.com/kanerix/lerpz-com/actions"><img src="https://img.shields.io/github/actions/workflow/status/kanerix/lerpz-com/pipeline.yaml?branch=main&style=flat-square&label=pipeline"></a>
    &nbsp;
    <a href="https://github.com/kanerix/lerpz-com"><img src="https://img.shields.io/badge/built_with-Rust-dea584.svg?style=flat-square"></a>
    &nbsp;
    <a href="https://github.com/kanerix/lerpz-com"><img src="https://img.shields.io/badge/and-SvelteKit-ff3e00.svg?style=flat-square"></a>
    &nbsp;
    <a href="https://lerpz.com"><img src="https://img.shields.io/badge/site-lerpz.com-3388c8.svg?style=flat-square"></a>
</p>

<br>

## What is Lerpz?

A monorepo containing shared libraries, services, and packages for the Lerpz
platform, an internal enterprise AI portal that provides chat interfaces, user
management, and organizational tools, all backed by Microsoft Entra ID
authentication.

Everything lives in one repository: two SvelteKit frontends, three Rust
services, the shared crates and packages they depend on, the Kubernetes and
Terraform definitions they deploy to, and the migrations that back them.

## Contents

- [Services](#services)
- [Architecture](#architecture)
- [Repository layout](#repository-layout)
- [Getting started](#getting-started)
- [Configuration](#configuration)
- [Local Kubernetes](#local-kubernetes)
- [Running the stack](#running-the-stack)
- [Database](#database)
- [Development](#development)
- [Deployment](#deployment)
- [Documentation](#documentation)

## Services

| Name | Role | Stack | Port |
|---|---|---|---|
| `www` | Company / landing site | SvelteKit (static) | 3000 |
| `app` | Product UI | SvelteKit | 3001 |
| `api` | Backend API | Rust / Axum | 4000 |
| `artoo` | In-app assistant | Rust | 4001 |
| `forge` | Agent infrastructure provisioner | Rust / Axum / kube | 5000 |
| `qdrant` | Vector database | Qdrant | 6333 / 6334 |
| `postgres` | Primary database | PostgreSQL | 6432 |
| `dragonfly` | Cache | Dragonfly (Redis-compatible) | 6379 |
| `minio` | Object storage | MinIO | 6000 / 6001 |

Ports 3000 to 3999 are web pages, 4000 to 4999 are publicly reachable APIs,
5000 to 5999 are internal services, and 6000 to 6999 is third-party
infrastructure. The infrastructure ports above are what you connect to **from
your machine**; in-network those containers keep their vendor defaults
(`postgres:5432`, `minio:9000`). See [docs/NAMING.md](docs/NAMING.md) for the
naming rules and the conventions for adding a new service.

## Architecture

`artoo` is the app's main agent: it answers questions and helps users find
their way around the product's features. The product UI does not call it yet.

`api`, `artoo` and `forge` each validate Entra ID tokens. The browser sends its
token to the public services, and `api` forwards agent management requests to
the internal Forge service. Traefik also asks Forge to authorise runtime
requests. Model traffic goes through [Portkey](https://portkey.ai) rather than
to a provider directly. `www` is fully static and depends on nothing. Forge
talks to a cluster rather than to the local infrastructure, which is why it
sits behind its own Compose profile.

See [docs/INFRA.md](docs/INFRA.md) for the topology diagram and where each
service runs.

## Repository layout

```
svc/          Deployable services (www, app, api, artoo, forge)
crates/       Shared Rust crates
packages/     Shared TypeScript packages
migrations/   sqlx migrations
k8s/          Kubernetes manifests and a kind cluster
terraform/    Infrastructure definitions
benchmarks/   Criterion benchmarks
docs/         Conventions and design notes
```

Shared Rust crates: `lerpz-ai`, `lerpz-axum`, `lerpz-jwt`, `lerpz-macros`,
`lerpz-metadata`, `lerpz-pwd`, `lerpz-utils`.

Shared TypeScript packages: `@lerpz/ui`, `@lerpz/biome-config`,
`@lerpz/typescript-config`.

## Getting started

### Prerequisites

Install [Git](https://git-scm.com/), a running [Docker](https://www.docker.com/)
daemon, [Nix](https://nixos.org/download/) and
[devenv](https://devenv.sh/getting-started/). Docker must have enough resources
to run a single-node kind cluster and the complete application stack.

Enter the development shell from the repository root:

```sh
devenv shell
```

The shell installs the Rust toolchain and Bun, then installs the workspace's Bun
dependencies. It also provides every repository CLI:

| CLI                                 | Use                                                     |
| ----------------------------------- | ------------------------------------------------------- |
| `git`                               | Source control, installed on the host                   |
| `docker`, `docker compose`          | Containers and the kind node, installed on the host     |
| `nix`, `devenv`                     | Reproducible development shell                          |
| `rustc`, `cargo`                    | Rust compiler and package tooling                       |
| `bun`                               | TypeScript dependencies, checks and development servers |
| `just`                              | Project task recipes                                    |
| `sqlx`                              | PostgreSQL migrations and offline query metadata        |
| `cargo-expand`                      | Rust macro expansion                                    |
| `mkcert`                            | Trusted local TLS certificates                          |
| `kubectl`                           | Kubernetes resources and logs                           |
| `helm`                              | Traefik installation                                    |
| `kind`                              | Local Kubernetes cluster                                |
| `mirrord`                           | Run a local Rust service against the cluster            |
| `terraform`                         | Cloud infrastructure                                    |
| `gh`                                | GitHub Actions, pull requests and issues                |
| `nixfmt`                            | Nix formatting                                          |
| `openssl`, `pkg-config`             | Native dependency discovery and TLS tooling             |
| `rg`, `fd`, `sd`, `jaq`, `ast-grep` | Repository search and structured edits                  |

Without devenv, install all of the tools above yourself, together with the Rust
edition 2024 nightly toolchain and Bun 1.4 or newer. Docker and Git remain host
prerequisites because the development shell does not provide them.

### Quick start

```sh
bun install
just dev
```

`just dev` starts the infrastructure containers, waits for them to become
healthy, then runs `api`, `artoo`, `app` and `www` on the host. One Ctrl-C
stops everything.

Run `just` on its own to list every recipe.

## Configuration

### 1. Configure Microsoft Entra ID

Register an application and expose a delegated API scope such as
`api://ai.lerpz.com/access_as_user`. Configure the browser application to use the
authorisation code flow with PKCE, then register these SPA redirect and logout
URIs for local development:

```text
http://localhost:3001
https://app.lerpz.local
```

Use the same tenant ID, client ID and named delegated scope in `app`, `api`,
`artoo` and `forge`. Do not use `.default`, an application permission or a client
secret for browser requests.

### 2. Generate TLS certificates

Use mkcert to create local certificates:

```sh
mkcert -install
mkcert -cert-file certs/cert.pem -key-file certs/key.pem \
  lerpz.local www.lerpz.local app.lerpz.local api.lerpz.local agent.lerpz.local \
  container.lerpz.local
```

### 3. Update your hosts file (for traefik)

Add these entries to `/etc/hosts`:

```
127.0.0.1 lerpz.local www.lerpz.local app.lerpz.local api.lerpz.local agent.lerpz.local container.lerpz.local
```

## Local Kubernetes

The `k8s/` directory runs the complete stack in a local single-node kind cluster.
Traefik accepts HTTPS traffic on the host, Kubernetes Services provide internal
DNS, and persistent volume claims hold local development data. Forge runs under
a namespaced service account with the RBAC needed to create agent runtimes and
memory volumes.

### Configure the services

Create the environment files used to generate Kubernetes Secrets:

```sh
cp svc/app/.env.example svc/app/.env.docker
cp svc/api/.env.example svc/api/.env.docker
cp svc/artoo/.env.example svc/artoo/.env.docker
cp svc/forge/.env.example svc/forge/.env.docker
```

Do not quote values in these files. Configure provider credentials, model names,
Google Cloud project details and your Entra registration, then use these local
cluster values:

| Service                 | Setting                         | Value                                                |
| ----------------------- | ------------------------------- | ---------------------------------------------------- |
| `app`                   | `PUBLIC_API_URL`                | `https://api.lerpz.local`                            |
| `app`                   | `PUBLIC_AGENT_RUNTIME_ORIGIN`   | `https://container.lerpz.local`                      |
| `app`                   | redirect and logout URIs        | `https://app.lerpz.local`                            |
| `api`, `artoo`, `forge` | `ALLOWED_ORIGINS`               | `https://app.lerpz.local`                            |
| `api`                   | `ADDR`                          | `0.0.0.0:4000`                                       |
| `artoo`                 | `ADDR`                          | `0.0.0.0:4001`                                       |
| `forge`                 | `ADDR`                          | `0.0.0.0:5000`                                       |
| `api`                   | `DATABASE_URL`                  | `postgres://lerpz:Password123@postgres:5432/primary` |
| `artoo`                 | `DATABASE_URL`                  | `postgres://lerpz:Password123@postgres:5432/primary` |
| `api`                   | `REDIS_URL`                     | `redis://dragonfly:6379`                             |
| `artoo`                 | `QDRANT_URL_GRPC`               | `http://qdrant:6334`                                 |
| `api`                   | `AWS_S3_ENDPOINT`               | `http://minio:9000`                                  |
| `api`                   | MinIO access key and secret     | `minioadmin` and `Password123`                       |
| `api`                   | `AWS_S3_BUCKET`                 | `lerpz`                                              |
| `api`                   | `FORGE_URL`                     | `http://forge:5000`                                  |
| `forge`                 | `KUBE_NAMESPACE`                | `lerpz`                                              |
| `forge`                 | `AGENT_RUNTIME_SERVICE_ACCOUNT` | `agent-runtime`                                      |
| `forge`                 | `AGENT_MEMORY_STORAGE_CLASS`    | `standard`                                           |
| `forge`                 | `AGENT_MEMORY_DEFAULT_SIZE`     | `1Gi`                                                |
| `forge`                 | temporary `AGENT_RUNTIME_IMAGE` | `nginxinc/nginx-unprivileged:alpine`                 |

Use the same Entra tenant ID, client ID and delegated scope in every service.
Prefix the browser variables with `PUBLIC_`. Remove `KUBECONFIG` from
`svc/forge/.env.docker`; the Forge pod uses its mounted service-account token.
The temporary Nginx runtime listens on port 8080 and proves provisioning and
routing, but does not implement an agent API.

### Create and deploy the cluster

After creating the environment files, certificates and hosts entry, run from the
repository root:

```sh
just k8s all
just k8s status
```

`all` creates `kind-lerpz`, installs Traefik, builds and loads the application
images, creates TLS and environment Secrets, and applies the manifests. Wait for
PostgreSQL to become ready, then apply the migrations:

```sh
just k8s migrate
```

If `kind-lerpz` already exists but the application has not been deployed, use
`just k8s bootstrap` instead of `all`. This keeps the cluster but performs the
initial image builds and deployment.

Open:

- <https://lerpz.local> for the company site
- <https://app.lerpz.local> for the product
- <https://api.lerpz.local> for the API
- <https://agent.lerpz.local> for Artoo
- `https://container.lerpz.local/<runtime-id>` for a provisioned runtime

Useful cluster commands:

| Command                                                                            | Does                                               |
| ---------------------------------------------------------------------------------- | -------------------------------------------------- |
| `just k8s status`                                                                  | Show application pods, Services and ingress routes |
| `kubectl -n lerpz get all`                                                         | Inspect namespace resources                        |
| `kubectl -n lerpz logs deploy/api -f`                                              | Follow API logs                                    |
| `kubectl -n lerpz logs deploy/forge -f`                                            | Follow Forge logs                                  |
| `just k8s secrets`                                                                 | Refresh Secrets after editing `.env.docker` files  |
| `kubectl -n lerpz rollout restart deploy/app deploy/api deploy/artoo deploy/forge` | Restart services after environment changes         |
| `just k8s down`                                                                    | Delete the cluster and all data stored in it       |

### Run Rust services locally with mirrord

Keep the cluster running, then replace one deployed Rust service with a locally
compiled process:

```sh
just k8s local api
just k8s local artoo
just k8s local forge
```

The recipe builds the selected debug binary locally, imports the target pod's
environment and steals its incoming traffic. Outgoing calls and DNS still use
the cluster, so the local API can reach `postgres`, `dragonfly` and `forge` by
their Kubernetes names. Press Ctrl-C to return traffic to the deployed pod.
Re-run the command after code changes. It does not rebuild or reload a container
image.

The local process handles real cluster requests and modifies real local cluster
data. Only run trusted code. The Svelte applications continue to use their normal
local development servers or deployed images; the mirrord recipes cover the Rust
services only.


## Running the stack

### Default mode (local development)

Start only the infrastructure services. If you followed the Traefik steps,
requests will be proxied to apps running on your local machine
(`localhost:3001` for `app`, `localhost:4000` for `api`):

```sh
just infra
```

Individual services can then be started on their own:

```sh
just www      # http://localhost:3000
just app      # http://localhost:3001
just api      # http://localhost:4000  (docs at /scalar)
just artoo    # http://localhost:4001  (docs at /scalar)
```

### Full mode (everything containerized)

Build and start every service in Docker:

```sh
just up
just down     # stop and remove, keeping volumes
```

### Forge

`forge` provisions agent infrastructure through the Kubernetes API, so it has no
self-contained local mode and sits behind its own profile:

```sh
just forge                              # on the host
docker compose --profile k8s up forge   # in a container
```

It mounts `~/.kube` read-only and exits on startup if no cluster answers. Because
a kind/minikube kubeconfig points at `127.0.0.1`, which inside the container is
the container itself, running `forge` in the kind cluster under [`k8s/`](k8s)
is usually the better path.

New deployments, runtime pods and persistent volume claims carry separate
creator and owner labels:

| Label | Value on creation |
|---|---|
| `lerpz.com/created-by-oid` | Caller's Entra object ID |
| `lerpz.com/created-by-tenant-id` | Caller's Entra tenant ID |
| `lerpz.com/owner-type` | `user` |
| `lerpz.com/owner-id` | Caller's Entra object ID |
| `lerpz.com/owner-tenant-id` | Caller's Entra tenant ID |

All identity values come from the validated token, not the request body. Every
runtime and volume endpoint requires non-empty `oid` and `tid` claims. For
app-only tokens, `user` ownership identifies the service principal rather than
a person. The API exposes `created_by_oid`, `created_by_tenant_id`, `owner_type`,
`owner_id` and `owner_tenant_id` separately.

Creator labels record attribution and do not grant access. Lists only return
Forge-managed resources with owner type `user` and owner IDs matching the caller.
Reads and deletes enforce the same checks plus the agent label, returning `404`
for inaccessible resources. Before provisioning a runtime, Forge checks that
its memory volume is user-owned by the caller. Deletes use UID and
resource-version preconditions to protect against changes after authorisation.

Team ownership and ownership-transfer endpoints are not implemented. Resources
labelled `owner-type=team`, with unknown owner types or without all three owner
labels are inaccessible. There is no fallback to creator-based access. Existing
resources are not backfilled. A cluster administrator must assign the owner
labels to existing deployments, their pod templates and memory volume claims.
For creator-only resources, the creator IDs can supply the owner IDs after
confirming that the creator is still the intended owner. Keep the creator labels
unchanged when assigning ownership.

Agent names remain unique across the Forge namespace, not per owner. Kubernetes
mounts volumes by name, so the ownership check cannot prevent a volume being
replaced by another owner's volume between authorisation and mounting. Strict
isolation against cross-owner name reuse requires stable resource IDs that are
never reused, or owner-scoped resource names.
Cluster access that can change these labels must also be restricted; the labels
are not an immutable audit trail.

New runtimes also get a ClusterIP Service and a Traefik HTTPS route. Configure
`AGENT_RUNTIME_ORIGIN` with the public HTTPS origin, `AGENT_RUNTIME_PORT` with the
default internal HTTP port and `AGENT_RUNTIME_TLS_SECRET` with a TLS secret in
Forge's namespace. The local Kubernetes manifest supplies example values; set
these variables when running Forge outside that deployment. Traefik's CRDs and
`websecure` entrypoint must be installed.

Runtime responses include `runtime_id`, `base_url` and `port`. The optional
creation field `port` overrides the configured internal port. The image must
listen on that port on `0.0.0.0`; Forge does not change the image's configuration.
All runtimes share the public HTTPS port. For example,
`https://container.lerpz.local/<runtime_id>/api/messages` reaches `/api/messages`
inside that runtime. The UUID and recorded URL survive pod restarts but change
when the runtime is recreated.

The browser sends its Forge-audience Entra access token. Traefik checks ownership
with Forge on each request, then strips the UUID prefix and credentials before
forwarding. CORS preflight is handled separately using `ALLOWED_ORIGINS`. The
runtime Service and route resources are garbage-collected with their deployment;
a networking failure triggers deployment rollback. Old runtimes without network
metadata must be recreated to receive a URL. Use a NetworkPolicy-enforcing CNI
when direct connections that bypass ingress must be blocked.

## Database

The project uses PostgreSQL with migrations managed by
[sqlx](https://github.com/launchbadge/sqlx). The schema includes tables for
conversations and messages supporting the AI chat feature.

```sh
just migrate            # apply pending migrations
just migration NAME     # create a new timestamped migration
just prepare            # regenerate the offline .sqlx cache
just psql               # open a shell against the running container
```

Compile-time query checks run offline by default, against the cached metadata in
`.sqlx`. Regenerate it with `just prepare` whenever a query changes.

## Development

| Command | Does |
|---|---|
| `just check` | Clippy, svelte-check and biome |
| `just fmt` | Format Rust and TypeScript in place |
| `just test` | Cargo test suite |
| `just bench` | Criterion benchmarks |
| `just build` | Release-build every service |
| `just openapi` | Regenerate the app's API client from `api`'s OpenAPI spec |
| `just logs` | Tail container logs |
| `just k8s` | Delegate to [k8s/justfile](k8s/justfile) |

## Deployment

| Workflow | Deploys |
|---|---|
| [`pipeline.yaml`](.github/workflows/pipeline.yaml) | Detects changed services and fans out to the others |
| [`deploy-container.yaml`](.github/workflows/deploy-container.yaml) | `app`, `api`, `artoo`, `forge` as containers |
| [`deploy-gh-page.yaml`](.github/workflows/deploy-gh-page.yaml) | `www` to GitHub Pages |

`pipeline.yaml` also runs a `check` job on every push and pull request, which
calls [`check.yaml`](.github/workflows/check.yaml) for the Rust and TypeScript
checks. The deploy jobs wait for it to pass.

Only changed services are deployed, and `www` publishes from `main` only. The
container deploy jobs are currently commented out in `pipeline.yaml`, so `www`
is the only service the pipeline actually ships.

## Documentation

- [docs/NAMING.md](docs/NAMING.md): service naming and port conventions
- [docs/BRAND.md](docs/BRAND.md): typeface, colour tokens and frontend traits
- [docs/INFRA.md](docs/INFRA.md): infrastructure
- [docs/GUID.md](docs/GUID.md): identifiers
