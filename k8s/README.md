# Local Kubernetes (kind) setup

This directory runs the entire Lerpz stack on a local [kind](https://kind.sigs.k8s.io/)
cluster with **Traefik** as the ingress controller — the Kubernetes equivalent
of the root `docker-compose.yml`.

## Layout

```
k8s/
├── kind-config.yaml        # kind cluster with 80/443 host port mappings
├── traefik/values.yaml     # Traefik Helm values (hostPort, CRD provider)
├── justfile                # build → load → deploy helpers
└── manifests/
    ├── namespace.yaml
    ├── infra/              # postgres, dragonfly, qdrant, minio (+ init job)
    ├── apps/               # coruscant, kamino, artoo
    └── ingress/            # Traefik IngressRoutes per host
```

## Prerequisites

- [kind](https://kind.sigs.k8s.io/), [kubectl](https://kubernetes.io/docs/tasks/tools/), [helm](https://helm.sh/)
- [just](https://github.com/casey/just) for the task recipes
- Docker (kind runs the cluster inside Docker)
- [mkcert](https://github.com/FiloSottile/mkcert) for local TLS certs

## What changes vs. docker-compose

Two things must be adjusted in your app env files before deploying:

1. **Service hostnames.** Compose used `*.lerpz.local` network aliases; in the
   cluster, reach infra by its Kubernetes Service name (all in the `lerpz`
   namespace):

   | Setting            | Value                          |
   |--------------------|--------------------------------|
   | `DATABASE_URL`     | `postgres://lerpz:Password123@postgres:5432/primary` |
   | `REDIS_URL`        | `redis://dragonfly:6379`       |
   | `QDRANT_URL_GRPC`  | `http://qdrant:6334`           |
   | `AWS_S3_ENDPOINT`  | `http://minio:9000`            |

2. **Bind address.** `ADDR` must bind `0.0.0.0` (e.g. `0.0.0.0:3001`), not
   `127.0.0.1`, so the pod is reachable from the Service.

Keep a `svc/<name>/.env.docker` per service with these values; the justfile
turns them into Kubernetes Secrets.

## Bring-up

```sh
# 0. From the repo root, generate certs + hosts entries (same as the main README)
mkcert -cert-file certs/cert.pem -key-file certs/key.pem lerpz.local api.lerpz.local
# /etc/hosts:  127.0.0.1 lerpz.local api.lerpz.local agent.lerpz.local

cd k8s

# 1. Create the cluster and install Traefik
just cluster
just traefik

# 2. Build the app images and load them into the cluster (no registry needed)
just build
just load

# 3. Create the TLS + per-app env secrets
just tls
just secrets

# 4. Deploy infra, apps, and ingress
just deploy

# 5. Run database migrations once postgres is ready
just migrate

# 6. Check status
just status
```

Or run the whole chain at once with `just all` (after creating the env files).

Then browse to <https://lerpz.local>, <https://api.lerpz.local>, and
<https://agent.lerpz.local>.

Tear everything down with `just down`.

## How it fits together

- **kind-config.yaml** labels the control-plane node `ingress-ready=true` and
  maps host ports 80/443 into it.
- **traefik/values.yaml** runs Traefik as a DaemonSet that binds those host
  ports and enables the Kubernetes CRD provider (for `IngressRoute`).
- **manifests/ingress** declares one `IngressRoute` per host, terminating TLS
  with the `lerpz-tls` secret — the direct analogue of the Traefik router
  labels in `docker-compose.yml`.

## Notes / next steps

- `infra/secrets.yaml` contains throwaway dev credentials matching compose. Do
  not reuse this pattern outside your machine.
- Infra runs as single-replica StatefulSets with 1Gi PVCs on kind's default
  `standard` (local-path) storage class.
- After changing app code, re-run `just build load` then
  `kubectl -n lerpz rollout restart deploy/<svc>`.
- For a tighter inner loop consider [Tilt](https://tilt.dev/) or
  [Skaffold](https://skaffold.dev/), which automate the build→load→restart cycle.
