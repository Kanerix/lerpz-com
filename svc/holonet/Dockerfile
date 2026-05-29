FROM oven/bun:1-slim AS base


FROM base AS deps
WORKDIR /app

COPY package.json bun.lock ./
COPY svc/holonet/package.json ./svc/holonet/
COPY packages/ui/package.json ./packages/ui/
COPY packages/typescript-config/package.json ./packages/typescript-config/

RUN --mount=type=cache,id=bun,target=/root/.bun/install/cache \
    bun install --frozen-lockfile


FROM base AS builder
WORKDIR /app

COPY --from=deps /app/node_modules ./node_modules

COPY package.json bun.lock ./
COPY svc/holonet ./svc/holonet
COPY packages/ui ./packages/ui
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

RUN --mount=type=cache,id=vite,target=/app/svc/holonet/.vite-cache \
    bun --filter holonet run build


FROM base AS runner
WORKDIR /app

ENV NODE_ENV=production \
    PORT=3000

RUN addgroup --system --gid 1001 app && \
    adduser --system --uid 1001 --ingroup app lerpz

COPY --from=builder --chown=lerpz:app /app/svc/holonet/build ./build
COPY --from=builder --chown=lerpz:app /app/svc/holonet/package.json ./package.json

RUN --mount=type=cache,id=bun,target=/root/.bun/install/cache \
    bun install --production --frozen-lockfile

USER lerpz

EXPOSE 3000

CMD ["bun", "build/index.js"]
