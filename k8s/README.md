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
| `AGENT_RUNTIME_PORT` | `8080` | Fixed port inside every runtime container |
| `AGENT_RUNTIME_TLS_SECRET` | `lerpz-tls` | TLS secret in the runtime namespace |

`container.lerpz.local` is an example, not a required hostname. Set the origin
in the manifest to your host and align DNS, the certificate and Traefik with it.
An origin can include an external port, such as
`https://container.lerpz.local:8443`, if Traefik's `websecure` entrypoint and the
host port mappings are configured accordingly. This is separate from the
runtime container port. The supplied local setup shares HTTPS port 443 across
all runtimes, rather than allocating an external port per runtime.

## Agent management through the core API

The core API manages private agents at `/api/v1/agents` and retained memory at
`/api/v1/agent-memory`. It validates product parameters, constructs separate
internal Forge requests and translates the results into public agent responses.
It does not relay client bodies or Forge responses. The same validated bearer
token authenticates internal requests. Forge remains on its ClusterIP Service
with no public management ingress. It validates the token and enforces ownership
through Kubernetes resource labels, not caller-supplied identity headers. Direct
runtime traffic remains on `https://container.lerpz.local`.

| Method | Public path | Operation |
|--------|-------------|-----------|
| `POST` | `/api/v1/agents` | Create an agent and prepare its selected memory |
| `GET` | `/api/v1/agents` | List the caller's agents |
| `GET`, `DELETE` | `/api/v1/agents/{name}` | Read or delete an agent |
| `GET` | `/api/v1/agent-memory` | List the caller's retained memory |
| `GET`, `DELETE` | `/api/v1/agent-memory/{name}` | Read or remove memory for an agent name |

Agent creation accepts only `name`, `memory` (`new`, `existing` or `none`)
and optional `resource_limits` with `cpu_millicores` and `memory_mib` fields.
Supplied limits must be positive integers. RAM limits are separate from
persistent memory storage. Names use 1 to 40 lowercase letters, digits or
hyphens, starting and ending with a letter or digit.

The API converts limits into internal quantities and requests one replica.
Forge configuration selects the image, port, memory size and storage class.
There is no standalone public memory creation endpoint. Use `memory: "new"`
when creating an agent, or `memory: "existing"` to reuse memory owned by the
caller with the same agent name.

Public agent responses contain `name`, `status`, `memory`, `url` and
`created_at`, not deployments, images, ports, storage classes or ownership
labels. Retained memory responses contain `agent_name`, `status`, `in_use`
and `created_at`.

Deleting an agent keeps its memory. New memory is also kept if agent startup
fails. Check the agent and retained memory before retrying with `existing`.
Memory removal checks whether the matching agent is using it, but that check is
not atomic with concurrent agent creation.

Set `FORGE_URL=http://forge:5000` in `svc/api/.env.docker`, which `just secrets`
loads into `api-env`. The API manifest reads that Secret and does not supply
`FORGE_URL` itself. This required setting must be one trusted HTTP or HTTPS
origin, with no credentials, path, query or fragment. The API constructs
internal destinations itself and disables redirects, proxies and retries.

Supply these app settings through `svc/app/.env.docker`, which `just secrets`
loads into `app-env`:

| Setting | Local value |
|---------|-------------|
| `PUBLIC_API_URL` | `https://api.lerpz.local` |
| `PUBLIC_ENTRA_ID_SCOPE` | The full delegated API scope, shared for management and runtime access |
| `PUBLIC_AGENT_RUNTIME_ORIGIN` | Optional; `https://container.lerpz.local`, matching Forge's `AGENT_RUNTIME_ORIGIN` |

These are runtime values read through validated `publicEnv`, not build
arguments. The optional runtime origin is needed only for direct runtime
requests. Without it, do not send tokens to returned runtime URLs; management
still uses the API settings. It must be an HTTPS origin with no credentials,
path, query or fragment. See
[Agent management and runtime configuration](../docs/INFRA.md#agent-management-and-runtime-configuration)
for the full validation and Entra configuration contract.

API and Forge must use the same `ENTRA_ID_TENANT_ID` and `ENTRA_ID_CLIENT_ID`
values in `api-env` and `forge-env`, so the API bearer token has the audience
both services validate. Use the same delegated API scope for both services'
`ENTRA_ID_SCOPE` and the app's `PUBLIC_ENTRA_ID_SCOPE`. There is no separate
browser Forge scope and no Entra client secret for these internal requests.
The API validates the token; Forge enforces the delegated permission in `scp`,
requires caller identity and checks ownership. App-only tokens are rejected by
Forge. Use a named scope, not `.default`. A future on-behalf-of (OBO) exchange
for distinct audiences is not implemented.

Set `ALLOWED_ORIGINS=https://app.lerpz.local` for both API and Forge. API needs
it for browser management requests; Forge still needs it to generate runtime
CORS middleware. Each setting accepts a single app origin, not a comma-separated
list. After updating Secrets with `just secrets`, restart `deploy/app`,
`deploy/api` and `deploy/forge`. Recreate runtimes if their CORS origin changed.

The frontend uses the public `agents` operations from the core API's
`/api/openapi.json`. Regenerate the browser client with `just openapi` from the
repository root. No Forge URL, scope or schema belongs in the browser client.

## Bring-up

```sh
# 0. From the repo root, trust the local CA, generate certs and add the hosts entries
mkcert -install
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

Forge has no public management, health or documentation route. For local
debugging or docs at `http://localhost:5000/scalar`, use
`kubectl -n lerpz port-forward svc/forge 5000:5000`. This loopback-only debugging
path is not the browser integration URL. The API calls Forge's ClusterIP
Service for management; Traefik also calls it internally for runtime ForwardAuth.

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
`https://configured-host/<random-uuid>`, exposed as `base_url` in Forge's internal
runtime responses alongside `runtime_id` and `port`. The core API exposes the
application URL as `url` in its public agent response without those internal fields.

The four middlewares run in this order:

1. CORS uses Forge's `ALLOWED_ORIGINS` and permits `GET`, `POST`, `PUT`, `PATCH`,
   `DELETE`, `HEAD` and `OPTIONS`. CORS preflight `OPTIONS` requests do not
   require a token.
2. ForwardAuth calls
   `http://forge.<KUBE_NAMESPACE>.svc:5000/api/v1/runtimes/{runtime_id}/authorize`,
   forwarding only the `Authorization` header from the request. This
   authenticated endpoint reads the Deployment's runtime and ownership labels
   to check the caller's ownership. This internal Service URL is unchanged;
   the request carries the same API bearer token used for management.
3. StripPrefix removes the UUID prefix, so `/uuid/api/messages` reaches the
   runtime as `/api/messages`.
4. Credential removal strips the bearer `Authorization` header and `Cookie`
   before the request reaches the runtime. It also removes `Set-Cookie` from
   runtime responses and sets `Cache-Control: no-store`.

The browser must send an Entra access token acquired for
`PUBLIC_ENTRA_ID_SCOPE`, with the shared API/Forge audience, not the runtime or
another API. No separate Forge scope is needed.

### Forge and runtime isolation

The `forge-ingress` NetworkPolicy in `manifests/apps/forge.yaml` selects
`app: forge` pods and permits ingress only on TCP 5000 from:

- Pods labelled `app: api` in the same namespace as Forge.
- Pods labelled `app.kubernetes.io/name=traefik` in the namespace labelled
  `kubernetes.io/metadata.name=traefik`, using the existing Traefik selectors.

It grants no runtime pods access to Forge and makes no egress changes.

The static `agent-runtime-ingress` NetworkPolicy in `manifests/apps/forge.yaml`
selects pods with both `app.kubernetes.io/managed-by=forge` and
`app.kubernetes.io/component=runtime`. It permits ingress only on the TCP named
port `http`, from pods labelled `app.kubernetes.io/name=traefik` in the namespace
labelled `kubernetes.io/metadata.name=traefik`. It makes no egress changes.

Both policies require a NetworkPolicy-enforcing CNI. Kind's default CNI does
not enforce NetworkPolicies, and this setup does not install a replacement.
Locally, neither the restriction on access to Forge nor the protection against
bypassing ForwardAuth by calling runtime Pods or Services directly is enforced.
Other NetworkPolicies are additive and must not grant broader access to Forge
or runtime pods.

### Create an agent and call its runtime

Send `POST https://api.lerpz.local/api/v1/agents` with an API access token
acquired for `PUBLIC_ENTRA_ID_SCOPE` and `Content-Type: application/json`.
This body creates an agent without persistent memory:

```json
{
  "name": "example",
  "memory": "none",
  "resource_limits": {
    "cpu_millicores": 500,
    "memory_mib": 512
  }
}
```

Omit `resource_limits` to leave limits to platform settings. Use `memory: "new"`
to create and mount persistent memory, or `memory: "existing"` to reuse it.
Do not send Forge fields such as `agent`, `image`, `replicas`, `mount_memory`,
`env` or `port`; the public API rejects unknown fields.

The runtime port is fixed by Forge's `AGENT_RUNTIME_PORT` configuration.
The local configuration uses `8080`. The configured image must bind
`0.0.0.0` on that port without per-request environment overrides. Forge does
not configure the server's bind address. A TCP readiness probe keeps the pod
out of the Service endpoints until that port accepts connections. A `201`
response and a non-null `url` do not guarantee readiness while the pod starts
and Traefik loads the route.

Before attaching a bearer token, validate the agent response's `url` against
`PUBLIC_AGENT_RUNTIME_ORIGIN` and require an application path. The public
response does not expose a separate runtime ID. Runtime requests must stay
within that URL's path prefix, reject redirects and omit cookies. Without a
configured trusted origin, do not send tokens to runtime URLs.

For an image that exposes `GET /api/messages`, append `/api/messages` to the
validated agent `url`. Runtime endpoints depend on the configured image;
agent creation does not imply a chat protocol.

Deploy from `/ai/agents`, manage agents at `/ai/agents/sessions` and manage
retained memory at `/ai/agents/memory`. Deleting an agent preserves its memory.
If creation fails, check both lists before retrying so retained memory can be
reused rather than recreated.

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
`base_url` and `port` internally, and null `url` in the public agent response.
Recreate them to get a public URL.

For an existing local cluster, add `container.lerpz.local` to `/etc/hosts` and
regenerate the certificate with the mkcert command above. The root README's
shorter host list does not cover this runtime endpoint. Build and load the
coordinated API, app and Forge images and configure their environment values
above, then run from `k8s/`:

```sh
just tls
just secrets
kubectl apply -f manifests/apps/api.yaml
kubectl apply -f manifests/apps/forge.yaml
kubectl apply -f manifests/ingress/ingressroutes.yaml
kubectl -n lerpz rollout restart deploy/app deploy/api deploy/forge
```

This refreshes the TLS and environment Secrets, including the API's
`FORGE_URL`, applies both ingress NetworkPolicies, and restarts the pods for
environment changes.
The three `AGENT_RUNTIME_*` settings remain in `manifests/apps/forge.yaml`;
`app-env` supplies `PUBLIC_AGENT_RUNTIME_ORIGIN` without an app manifest change.

If the draft public management route was applied manually, remove only that
resource with `kubectl -n lerpz delete ingressroute forge --ignore-not-found`.
Applying manifests does not delete removed resources. Keep the dynamic runtime
routes, runtime host entry and runtime TLS certificate coverage.

## Notes / next steps

- `infra/secrets.yaml` contains throwaway dev credentials matching compose. Do
  not reuse this pattern outside your machine.
- Infra runs as single-replica StatefulSets with 1Gi PVCs on kind's default
  `standard` (local-path) storage class.
- After changing app code, re-run `just build load` then
  `kubectl -n lerpz rollout restart deploy/<svc>`.
- For a tighter inner loop consider [Tilt](https://tilt.dev/) or
  [Skaffold](https://skaffold.dev/), which automate the build→load→restart cycle.
