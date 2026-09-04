# Naming

This document outlines the naming conventions used in the Lerpz.com platform.

## The rule

> **Platform services are named for their role. Products are named for their
> identity.**

`app`, `api`, and `www` are plumbing — they should say what they are and survive
a rewrite in a different language or framework. `artoo` is different: it is
something users talk to, with a name and a personality, the way an assistant
product has a name. That distinction is the only reason the service list is not
uniformly descriptive.

Third-party infrastructure (databases, cache, storage, etc.) keeps its
conventional technology name.

## Services

| Name | Role | Stack | Port | Host |
|---|---|---|---|---|
| `www` | Company / landing site | SvelteKit (static) | 3000 | `lerpz.com` |
| `app` | Product UI | SvelteKit | 3001 | `app.lerpz.com` |
| `api` | Core backend API | Rust / Axum | 4000 | `api.lerpz.com` |
| `artoo` | AI agent | Rust | 4001 | `agent.lerpz.com` |
| `forge` | Agent infrastructure provisioner | Rust / Axum / kube | 5000 | internal |

For anything public, the service name *is* the subdomain: `svc/app` →
`app.lerpz.local` → `app.lerpz.com`. There is no lookup table to keep in sync.

### `www` — Company site

Fully prerendered static HTML with no server runtime, so it can be hosted for
free on GitHub Pages or Azure Static Web Apps. Kept separate from `app` so
marketing copy deploys independently, caches aggressively, and never drags the
authentication bundle along with it.

### `app` — Product UI

The authenticated product surface. Everything behind a login lives here.

### `api` — Core backend API

The primary backend. When a second API becomes necessary, it is named for its
domain (`auth`, `billing`, `search`) rather than becoming `api2`; `api` remains
the core service.

### `artoo` — AI agent

R2-D2 is the most capable and resourceful droid in the galaxy. He acts
autonomously, assesses situations on the fly, interfaces with foreign systems,
and executes missions without being told exactly how. An AI agent that operates
independently, reasons over data, and returns results maps onto this kind of
self-directed capability — and, unlike the rest of the platform, it is a thing
users address by name.

Publicly reachable at `agent.lerpz.com`: it terminates a delegated Entra user
token (`access_as_user`) and applies CORS, because the browser streams from it
directly.

### `forge` — Agent infrastructure provisioner

Turns a request for agent capacity into the Kubernetes objects that back it:
persistent memory volumes (`PersistentVolumeClaim`) and the container runtimes
that mount them (`Deployment`). Named for what it does — it forges the
environment an agent runs in — and deliberately *not* named `orchestrator`,
`scheduler`, `controller`, or `operator`, all of which already mean something
specific in a Kubernetes cluster and would poison every grep.

Internal only: it holds RBAC over the namespace, so it has no ingress and is
reachable only from `api` and `artoo`. It is currently the platform's only
internal service.

## Port ranges

| Range | Used for |
|---|---|
| 3000–3999 | Web pages |
| 4000–4999 | Publicly reachable APIs |
| 5000–5999 | Internal services |
| 6000–6999 | Third-party infrastructure |

The range a service falls in tells you its exposure at a glance: anything on a
4000 port has an ingress route and is reachable from outside the cluster;
anything on a 5000 port is only callable service-to-service and should have no
ingress. `api` and `artoo` are both on 4000 ports because browsers call them
directly with a delegated user token; `forge` is on 5000 because only other
services call it.

### These ranges apply to *host* ports

There are two different ports in play, and only one of them is ours to allocate:

- **The host-published port** (`ports:` in `docker-compose.yml`) — every service
  shares your machine's single address space, so this is the only place ports
  genuinely collide. These follow the ranges above.
- **The container port** — each container and each Kubernetes Service has its
  own IP, so there is nothing to collide with. Third-party images keep their
  vendor default here, because that is what `pg_isready`, `psql`, `redis-cli`,
  and `mc` expect with no flags.

So Postgres is reachable at `localhost:6432` from your machine, but services
inside the network still address it as `postgres:5432`. The same split applies
to MinIO: `localhost:6000` from the host, `minio:9000` in-network.

| Service | Host | Container |
|---|---|---|
| `postgres` | 6432 | 5432 |
| `qdrant` | 6333 / 6334 | 6333 / 6334 |
| `dragonfly` | 6379 | 6379 |
| `minio` | 6000 / 6001 | 9000 / 9001 |

Qdrant and Dragonfly need no remapping — their conventional ports already sit in
the 6000 range. Postgres keeps its recognisable `432` suffix, and MinIO's API
and console stay adjacent.

## Rules for new services

1. **Name the role, not the technology.** `api`, not `axum`. The name should
   survive a rewrite.
2. **Public service → its subdomain.** `docs`, `status`, `cdn`, `admin`.
3. **Internal service → a role noun, on a 5000 port.** `forge`, `worker`,
   `gateway`, `events`.
4. **Name by domain once there is more than one API.** Not `api2` or `api-v2`,
   but `auth`, `billing`, `search`.
5. **Lowercase, ASCII, one word.** Hyphens only when no single word exists.
   These names become crate names, container names, hostnames, and DNS labels.
6. **Avoid names that collide with infrastructure vocabulary.** Nothing should
   be called `ingress`, `operator`, `controller`, or `scheduler`.
7. **Reserve product names for things users perceive as an entity.** `artoo`
   qualifies; almost nothing else will.
