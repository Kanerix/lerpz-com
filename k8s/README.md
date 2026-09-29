# Local Kubernetes (kind) setup

This directory runs the entire Lerpz stack on a local [kind](https://kind.sigs.k8s.io/)
cluster with **Traefik** as the ingress controller, the Kubernetes equivalent
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
    ├── apps/               # www, app, api, artoo, forge
    └── ingress/            # Traefik IngressRoutes per host
```

## Prerequisites

- [kind](https://kind.sigs.k8s.io/), [kubectl](https://kubernetes.io/docs/tasks/tools/), [helm](https://helm.sh/)
- [just](https://github.com/casey/just) for the task recipes
- Docker (kind runs the cluster inside Docker)
- [mkcert](https://github.com/FiloSottile/mkcert) for local TLS certs

## What changes vs. docker-compose

Adjust these settings in your app env files before deploying:

1. **Service hostnames.** Compose used `*.lerpz.local` network aliases; in the
   cluster, reach infra by its Kubernetes Service name (all in the `lerpz`
   namespace):

   | Setting            | Value                          |
   |--------------------|--------------------------------|
   | `DATABASE_URL`     | `postgres://lerpz:Password123@postgres:5432/primary` |
   | `REDIS_URL`        | `redis://dragonfly:6379`       |
   | `QDRANT_URL_GRPC`  | `http://qdrant:6334`           |
   | `AWS_S3_ENDPOINT`  | `http://minio:9000`            |

2. **Bind address.** `ADDR` must bind `0.0.0.0` (e.g. `0.0.0.0:4000`), not
   `127.0.0.1`, so the pod is reachable from the Service.

3. **Forge's kubeconfig.** Remove `KUBECONFIG` from `svc/forge/.env.docker`
   before creating the secret. In-cluster, `forge` authenticates with the
   ServiceAccount token mounted into its pod; `apps/forge.yaml` declares that
   ServiceAccount along with the namespaced Role that lets it manage
   `persistentvolumeclaims` and `deployments`, and create Services and Traefik
   IngressRoutes and Middlewares. Forge needs only `create` for Services,
   IngressRoutes and Middlewares, not `get`, `list` or `delete`. Kubernetes
   garbage-collects these network objects through their Deployment owner
   references.

Keep a `svc/<name>/.env.docker` per service with these values; the justfile
turns them into Kubernetes Secrets. `www` has none. It is fully static.

Forge also requires these non-secret settings, supplied directly in
`manifests/apps/forge.yaml` so existing `forge-env` secrets can still be used:

| Setting | Local example | Purpose |
|---------|---------------|---------|
| `AGENT_RUNTIME_ORIGIN` | `https://container.lerpz.local` | Public HTTPS origin, without a path, query or fragment |
| `AGENT_RUNTIME_PORT` | `8080` | Default port inside the runtime container |
| `AGENT_RUNTIME_TLS_SECRET` | `lerpz-tls` | TLS secret in the runtime namespace |

`container.lerpz.local` is an example, not a required hostname. Set the origin
in the manifest to your host and align DNS, the certificate and Traefik with it.
An origin can include an external port, such as
`https://container.lerpz.local:8443`, if Traefik's `websecure` entrypoint and the
host port mappings are configured accordingly. This is separate from the
runtime container port. The supplied local setup shares HTTPS port 443 across
all runtimes, rather than allocating an external port per runtime.

## Bring-up

```sh
# 0. From the repo root, generate certs and add the hosts entries
mkcert -cert-file certs/cert.pem -key-file certs/key.pem \
  lerpz.local www.lerpz.local app.lerpz.local api.lerpz.local agent.lerpz.local \
  container.lerpz.local
# /etc/hosts:  127.0.0.1 lerpz.local www.lerpz.local app.lerpz.local api.lerpz.local agent.lerpz.local container.lerpz.local

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

Then browse to <https://lerpz.local> (company site), <https://app.lerpz.local>,
<https://api.lerpz.local>, and <https://agent.lerpz.local>.

`forge` has no public management IngressRoute. It holds RBAC over the namespace
and stays on its ClusterIP Service, which Traefik also calls for runtime
ForwardAuth. Reach it for debugging with
`kubectl -n lerpz port-forward svc/forge 5000:5000`.

Tear everything down with `just down`.

## How it fits together

- **kind-config.yaml** labels the control-plane node `ingress-ready=true` and
  maps host ports 80/443 into it.
- **traefik/values.yaml** runs Traefik as a DaemonSet that binds those host
  ports and enables the Kubernetes CRD provider (for `IngressRoute`).
- **manifests/ingress** declares static app `IngressRoute` resources, terminating
  TLS with the `lerpz-tls` secret, the direct analogue of the Traefik router
  labels in `docker-compose.yml`. Runtime routes are created by Forge instead.

## Runtime networking

For each runtime, Forge creates a ClusterIP Service, a
`traefik.io/v1alpha1` IngressRoute and four Middleware resources in the same
`KUBE_NAMESPACE` as the runtime Deployment. The route uses `websecure` and
`AGENT_RUNTIME_TLS_SECRET`. Its public URL is
`https://configured-host/<random-uuid>`, exposed as `base_url` in the runtime
create, read and list responses alongside `runtime_id` and `port`.

The four middlewares run in this order:

1. CORS uses Forge's `ALLOWED_ORIGINS` and permits `GET`, `POST`, `PUT`, `PATCH`,
   `DELETE`, `HEAD` and `OPTIONS`. CORS preflight `OPTIONS` requests do not
   require a token.
2. ForwardAuth calls
   `http://forge.<KUBE_NAMESPACE>.svc:5000/api/v1/runtimes/{runtime_id}/authorize`,
   forwarding only the `Authorization` header from the request. This
   authenticated endpoint reads the Deployment's runtime and ownership labels
   to check the caller's ownership. It is not a public management route.
3. StripPrefix removes the UUID prefix, so `/uuid/api/messages` reaches the
   runtime as `/api/messages`.
4. Credential removal strips the bearer `Authorization` header and `Cookie`
   before the request reaches the runtime. It also removes `Set-Cookie` from
   runtime responses and sets `Cache-Control: no-store`.

The browser must send an Entra access token whose audience is Forge, not the
runtime or another API. Configure Forge's `ALLOWED_ORIGINS` to permit the
frontend origin, for example `https://app.lerpz.local`.

### Runtime isolation

The static `agent-runtime-ingress` NetworkPolicy in `manifests/apps/forge.yaml`
selects pods with both `app.kubernetes.io/managed-by=forge` and
`app.kubernetes.io/component=runtime`. It permits ingress only on the TCP named
port `http`, from pods labelled `app.kubernetes.io/name=traefik` in the namespace
labelled `kubernetes.io/metadata.name=traefik`. It makes no egress changes.

A NetworkPolicy-enforcing CNI is required to prevent another arbitrary runtime
from bypassing ForwardAuth by calling a runtime Pod or Service directly. The
stock cluster setup here does not install an enforcing CNI, so this isolation
is not enforced locally by that setup. Other NetworkPolicies are additive and
must not grant broader access to runtime pods.

### Create and call a runtime

Send an authenticated `POST /api/v1/runtimes` to Forge from an in-cluster caller,
or to `http://localhost:5000/api/v1/runtimes` through the debugging port-forward
above. For example, this body uses the configured `AGENT_RUNTIME_IMAGE` and
skips mounting a memory volume:

```json
{
  "agent": "example",
  "port": 8080,
  "mount_memory": false,
  "env": {
    "ADDR": "0.0.0.0:8080"
  }
}
```

The optional API field `port: Option<u16>` defaults to `AGENT_RUNTIME_PORT` when
omitted or null. `8080` is an explicit example, not an inferred image port.
The runtime must bind `0.0.0.0` on the selected port itself. A TCP readiness
probe keeps it out of the Service endpoints until that port accepts connections.
The URL can take time to become reachable while the pod starts and Traefik
loads the route. Forge does not configure the server's bind address. The sample `ADDR` works only for an image
that honours it; use the chosen image's configuration. If `mount_memory` is
omitted, it defaults to `true` and the caller must provision the memory volume
first.

Pass the returned `base_url` and an MSAL access token acquired for Forge to the
frontend request. For a runtime exposing `GET /api/messages`:

```ts
async function fetchMessages(baseUrl: string | null, forgeAccessToken: string) {
  if (!baseUrl) throw new Error("Recreate this runtime to enable networking");

  const response = await fetch(`${baseUrl}/api/messages`, {
    headers: { Authorization: `Bearer ${forgeAccessToken}` },
    credentials: "omit",
  });
  if (!response.ok) throw new Error(`Runtime request failed: ${response.status}`);
  return response.json();
}
```

### Lifecycle and existing clusters

The Service, IngressRoute and all four Middlewares have `ownerReferences` to the
runtime Deployment. Deleting that Deployment garbage-collects its network
objects. If networking creation fails, Forge rolls back the runtime Deployment
so any network objects already created are also collected.

The UUID URL survives pod restarts but changes when a runtime is deleted and
recreated. Forge stores `base_url` in a Deployment annotation at creation.
Changing `AGENT_RUNTIME_ORIGIN` does not change existing URLs in API responses
or update their routes. To use a new origin, recreate the runtimes so their
routes are reprovisioned.

Existing runtimes created without networking report null `runtime_id`,
`base_url` and `port`. Recreate them to get a public URL.

For an existing local cluster, add `container.lerpz.local` to `/etc/hosts` and
regenerate the certificate with the mkcert command above. Build and load the
updated Forge image, then run from `k8s/`:

```sh
just tls
kubectl apply -f manifests/apps/forge.yaml
```

This refreshes the TLS secret and applies the runtime NetworkPolicy alongside
the namespaced Role and Forge Deployment, including the required env settings.
Existing `forge-env` secrets do not need the three new values.

## Notes / next steps

- `infra/secrets.yaml` contains throwaway dev credentials matching compose. Do
  not reuse this pattern outside your machine.
- Infra runs as single-replica StatefulSets with 1Gi PVCs on kind's default
  `standard` (local-path) storage class.
- After changing app code, re-run `just build load` then
  `kubectl -n lerpz rollout restart deploy/<svc>`.
- For a tighter inner loop consider [Tilt](https://tilt.dev/) or
  [Skaffold](https://skaffold.dev/), which automate the build→load→restart cycle.
