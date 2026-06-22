FROM oven/bun:1-slim AS base


FROM base AS builder
WORKDIR /app

# Install dependencies first so this layer is cached unless a manifest changes.
# Bun uses an isolated node_modules layout (a central store at node_modules/.bun
# plus per-package symlink trees), so installing from the manifests alone
# produces the nested node_modules that the build relies on.
COPY package.json bun.lock ./
COPY svc/coruscant/package.json ./svc/coruscant/
COPY packages/ui/package.json ./packages/ui/
COPY packages/biome-config/package.json ./packages/biome-config/
COPY packages/typescript-config/package.json ./packages/typescript-config/

RUN --mount=type=cache,id=bun,target=/root/.bun/install/cache \
    bun install --frozen-lockfile

# Copy source and build.
COPY svc/coruscant ./svc/coruscant
COPY packages/ui ./packages/ui
COPY packages/biome-config ./packages/biome-config
COPY packages/typescript-config ./packages/typescript-config

ENV SKIP_ENV_VALIDATION=1

ARG PUBLIC_API_URL
ARG PUBLIC_ENTRA_ID_TENANT_ID
ARG PUBLIC_ENTRA_ID_CLIENT_ID
ARG PUBLIC_ENTRA_ID_SCOPE
ARG PUBLIC_ENTRA_ID_REDIRECT_URI
ARG PUBLIC_ENTRA_ID_LOGOUT_URI

ENV PUBLIC_API_URL=$PUBLIC_API_URL \
    PUBLIC_ENTRA_ID_TENANT_ID=$PUBLIC_ENTRA_ID_TENANT_ID \
    PUBLIC_ENTRA_ID_CLIENT_ID=$PUBLIC_ENTRA_ID_CLIENT_ID \
    PUBLIC_ENTRA_ID_SCOPE=$PUBLIC_ENTRA_ID_SCOPE \
    PUBLIC_ENTRA_ID_REDIRECT_URI=$PUBLIC_ENTRA_ID_REDIRECT_URI \
    PUBLIC_ENTRA_ID_LOGOUT_URI=$PUBLIC_ENTRA_ID_LOGOUT_URI

RUN --mount=type=cache,id=vite,target=/app/svc/coruscant/.vite-cache \
    cd svc/coruscant && bun run build


FROM base AS prod-deps
WORKDIR /app

# Production-only install, again from manifests so the isolated layout is intact.
COPY package.json bun.lock ./
COPY svc/coruscant/package.json ./svc/coruscant/
COPY packages/ui/package.json ./packages/ui/
COPY packages/biome-config/package.json ./packages/biome-config/
COPY packages/typescript-config/package.json ./packages/typescript-config/

RUN --mount=type=cache,id=bun,target=/root/.bun/install/cache \
    bun install --production --frozen-lockfile


FROM base AS runner
WORKDIR /app/svc/coruscant

ENV NODE_ENV=production \
    PORT=3000

RUN groupadd --system --gid 1001 app && \
    useradd --system --no-log-init --uid 1001 --gid app lerpz

# Copy the whole production dependency tree so the isolated store and the
# per-package symlinks keep their relative structure intact.
COPY --from=prod-deps --chown=lerpz:app /app /app

# Built server output (adapter-node), placed next to its node_modules.
COPY --from=builder --chown=lerpz:app /app/svc/coruscant/build ./build

USER lerpz

EXPOSE 3000

CMD ["bun", "build/index.js"]
